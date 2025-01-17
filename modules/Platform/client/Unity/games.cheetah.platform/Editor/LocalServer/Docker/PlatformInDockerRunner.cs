using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Net.Sockets;
using System.Threading;
using System.Threading.Tasks;
using Cheetah.Platform.Editor.LocalServer.Applications;
using Cheetah.Platform.Editor.LocalServer.SharedConfig;
using Docker.DotNet;
using Docker.DotNet.Models;
using UnityEditor;
using UnityEngine;

namespace Cheetah.Platform.Editor.LocalServer.Docker
{
    /// <summary>
    /// Запускаем серверные приложения в Docker
    /// </summary>
    public class PlatformInDockerRunner : IDisposable
    {
        public delegate void ChangeStatus(Status status);

        private const string unityProjectId = "cheetah";

        private Status _status;

        private readonly DockerClient docker;

        private readonly DockerLogWatcher logWatcher;
        
        private readonly SystemApplicationsConfigurator systemApplicationsConfigurator;


        public PlatformInDockerRunner(SystemApplicationsConfigurator systemApplicationsConfigurator)
        {
            this.systemApplicationsConfigurator = systemApplicationsConfigurator;
            Status = Status.Unknown;

            DockerClientConfiguration dockerClientConfiguration;
            try
            {
                var dockerUrl = Environment.GetEnvironmentVariable("DOCKER_HOST");
                dockerClientConfiguration = new DockerClientConfiguration(new Uri(dockerUrl));
            }
            catch (ArgumentException)
            {
                dockerClientConfiguration = new DockerClientConfiguration();
            }
            docker = dockerClientConfiguration.CreateClient();

            logWatcher = new DockerLogWatcher(docker, systemApplicationsConfigurator.ShowInfoLogs);
            AssemblyReloadEvents.beforeAssemblyReload += Dispose;

            var taskScheduler = TaskScheduler.FromCurrentSynchronizationContext();
            Task.Factory.StartNew(
                async () => { await ResolveState(); },
                CancellationToken.None,
                TaskCreationOptions.None,
                taskScheduler);
        }

        public Status Status
        {
            get => _status;
            private set
            {
                _status = value;
                OnStatusChange?.Invoke(_status);
            }
        }


        public void Dispose()
        {
            logWatcher?.Dispose();
        }

        public event ChangeStatus OnStatusChange;


        /// <summary>
        ///     Определение состояния после перезапуска Unity
        ///     - проверяем все ли контейнеры запущены
        /// </summary>
        /// <returns></returns>
        public async Task ResolveState()
        {
            if (Status != Status.Unknown) return;

            try
            {
                var serverApplications = Registry.GetApplications().ToDictionary(c => c.ContainerName);
                var existContainers = await docker.Containers.ListContainersAsync(new ContainersListParameters
                {
                    Filters = new Dictionary<string, IDictionary<string, bool>>
                    {
                        ["label"] = new Dictionary<string, bool>
                        {
                            [DockerContainerBuilder.DockerProjectLabel + "=" + unityProjectId] = true
                        }
                    },
                    All = true
                });


                foreach (var existContainer in existContainers)
                {
                    var label = existContainer.Labels[DockerContainerBuilder.DockerNameLabel];
                    if (label != null)
                        if (serverApplications.TryGetValue(label, out var serverApplication))
                            if (serverApplication.DockerImage.Ref == existContainer.Image)
                            {
                                serverApplications.Remove(label);
                                logWatcher.WatchLogs(existContainer.ID, serverApplication);
                            }
                }

                Status = serverApplications.Any() ? Status.Stopped : Status.Started;
            }
            catch (HttpRequestException e)
            {
                Debug.LogException(e);
                Status = Status.Fail;
                throw;
            }
            catch (Exception e)
            {
                Debug.LogError(e);
                Status = Status.Fail;
            }
        }

        public async Task Restart(IDockerProgressListener progressListener)
        {
            Status = Status.Starting;
            try
            {
                progressListener.SetProgressTitle("remove previous");
                progressListener.SetProgress(0);

                // удаляем все, что было запущено
                await Remove(docker, progressListener);

                progressListener.SetProgressTitle("create network");
                progressListener.SetProgress(0);
                var network = await CreateNetwork(unityProjectId, docker);

                var progress = 0;
                var serverApplications = Registry.GetApplications();
                await LaunchPostgresql(progressListener, serverApplications, progress, network);
                
                var deltaProgress = 100 / serverApplications.Count; 
                var done = false;
                var launched = new HashSet<string>();
                var clonedServerApplications = new List<ServerApplication>(serverApplications);
                while (!done)
                {
                    done = true;

                    progressListener.SetProgressTitle("starting services, remaining " + serverApplications.Count);
                    progressListener.SetProgress(progress);
                    foreach (var serverApplication in serverApplications.Where(application =>
                        {
                            var dependencies = new HashSet<string>(application.Dependencies);
                            dependencies.RemoveWhere(p => launched.Contains(p));
                            return dependencies.Count == 0;
                        }
                    ).ToList())
                    {
                        await Launch(serverApplication, network.ID, progressListener);
                        serverApplications.Remove(serverApplication);
                        launched.Add(serverApplication.ContainerName);
                        done = false;
                        progress += deltaProgress;
                    }


                    if (done && serverApplications.Count > 0)
                    {
                        Debug.LogError("Cannot resolve dependencies for containers");
                        await Remove(docker, progressListener);
                        Status = Status.Fail;
                        return;
                    }
                }

                await LaunchGrpcProxy(progressListener, clonedServerApplications, network);

                Status = Status.Started;
            }
            catch (HttpRequestException e)
            {
                Status = Status.Disconnected;
                throw new DockerConnectException(e);
            }
            catch (SocketException e)
            {
                Status = Status.Disconnected;
                throw new DockerConnectException(e);
            }
            catch (Exception)
            {
                Status = Status.Fail;
                throw;
            }
            finally
            {
                if (Status == Status.Fail)
                {
                    if (!systemApplicationsConfigurator.KeepFailedContainers)
                    {
                        await Remove(docker, progressListener);
                    }
                    Status = Status.Fail;
                }
            }
        }

        private async Task LaunchGrpcProxy(IDockerProgressListener progressListener, List<ServerApplication> serverApplications, NetworksCreateResponse network)
        {
            var grpcProxyApplication = new GrpcProxyApplication(systemApplicationsConfigurator, serverApplications);
            await Launch(grpcProxyApplication, network.ID, progressListener);
        }

        private async Task LaunchPostgresql(IDockerProgressListener progressListener, List<ServerApplication> serverApplications, int progress,
            NetworksCreateResponse network)
        {
            var applicationWithPostgresql = serverApplications.FindAll(app => app.PostgresDatabase!=null).Select(a=>a.PostgresDatabase).ToList();
            if (applicationWithPostgresql.Count > 0)
            {
                progressListener.SetProgressTitle("starting postgresql database");
                progressListener.SetProgress(progress);
                await Launch(new PostgreSqlApplication(applicationWithPostgresql), network.ID, progressListener);
            }
        }


        private async Task<string> Launch(ServerApplication serverApplication, string networkId, IDockerProgressListener progressListener)
        {
            await ImagePull(serverApplication.DockerImage, progressListener, serverApplication.ContainerName);
            var dockerContainerBuilder = new DockerContainerBuilder(serverApplication.ContainerName, serverApplication.DockerImage);
            serverApplication.ConfigureDockerContainerBuilder(dockerContainerBuilder);
            if (serverApplication.PostgresDatabase!=null) serverApplication.ConfigurePostgresEnv(dockerContainerBuilder);

            var createContainerResponse =
                await docker.Containers.CreateContainerAsync(dockerContainerBuilder.BuildDockerConfig(networkId, unityProjectId));
            logWatcher.WatchLogs(createContainerResponse.ID, serverApplication);
            var containerStarted = await docker.Containers.StartContainerAsync(createContainerResponse.ID, new ContainerStartParameters());
            if (!containerStarted) throw new Exception("Container " + serverApplication.ContainerName + " starting fail");
            await WaitingForLaunch.Wait(docker, createContainerResponse.ID, serverApplication.ContainerName);
            return createContainerResponse.ID;
        }

        private async Task ImagePull(DockerImage dockerImage, IDockerProgressListener progressListener, string title)
        {
            var listImagesParameters = new ImagesListParameters
            {
                Filters = new Dictionary<string, IDictionary<string, bool>>
                {
                    ["reference"] = new Dictionary<string, bool>
                    {
                        [dockerImage.Ref] = true
                    }
                }
            };
            if ((await docker.Images.ListImagesAsync(listImagesParameters)).Count > 0)
            {
                return;
            }

            await docker.Images.CreateImageAsync(new ImagesCreateParameters
            {
                FromImage = dockerImage.Ref
            },
                null,
                new ImageCreateProgress(progressListener, title));
        }


        private static async Task<NetworksCreateResponse> CreateNetwork(string instanceId, DockerClient docker)
        {
            var networkConfig = new NetworksCreateParameters
            {
                Name = "cheetah_platform_network_" + instanceId,
                Labels = new Dictionary<string, string> { [DockerContainerBuilder.DockerProjectLabel] = instanceId }
            };
            return await docker.Networks.CreateNetworkAsync(networkConfig);
        }

        private async Task Remove(DockerClient docker, IDockerProgressListener progressListener)
        {
            // удаляем контейнеры
            var filters = new Dictionary<string, IDictionary<string, bool>>
            {
                ["label"] = new Dictionary<string, bool>
                {
                    [DockerContainerBuilder.DockerProjectLabel + "=" + unityProjectId] = true
                }
            };
            var containers = await docker.Containers.ListContainersAsync(new ContainersListParameters
            {
                All = true,
                Filters = filters
            });
            if (containers.Count > 0)
            {
                var progress = 0;
                var progressDelta = 80 / containers.Count;
                foreach (var container in containers)
                {
                    if (container.State == "Up")
                    {
                        progressListener.SetProgressTitle("stop " + container.Names.First());
                        progressListener.SetProgress(progress);
                        await docker.Containers.StopContainerAsync(container.ID, new ContainerStopParameters
                        {
                            WaitBeforeKillSeconds = 1
                        });
                    }

                    progressListener.SetProgressTitle("delete " + container.Names.First());
                    progressListener.SetProgress(progress);
                    await docker.Containers.RemoveContainerAsync(container.ID, new ContainerRemoveParameters
                    {
                        Force = true,
                        RemoveLinks = false,
                        RemoveVolumes = false
                    });
                    progress += progressDelta;
                }
            }

            progressListener.SetProgressTitle("stop network");
            progressListener.SetProgress(80);

            var networks = await docker.Networks.ListNetworksAsync(new NetworksListParameters
            {
                Filters = filters
            });
            foreach (var network in networks) await docker.Networks.DeleteNetworkAsync(network.ID);

            progressListener.SetProgressTitle("stopped network");
            progressListener.SetProgress(100);
        }

        public async Task Stop(IDockerProgressListener progressListener)
        {
            try
            {
                Status = Status.Stopping;
                progressListener.SetProgressTitle("stop");
                progressListener.SetProgress(0);
                await Remove(docker, progressListener);
                progressListener.SetProgressTitle("stopped");
                progressListener.SetProgress(100);
            }
            catch (HttpRequestException e)
            {
                throw new DockerConnectException(e);
            }
            finally
            {
                Status = Status.Stopped;
            }
        }
    }

    public enum Status
    {
        Unknown,
        Starting,
        Started,
        Stopping,
        Stopped,
        Fail,
        Disconnected
    }
}