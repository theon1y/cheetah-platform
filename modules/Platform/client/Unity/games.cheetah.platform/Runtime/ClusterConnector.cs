using System;
using System.Net.Http;
using System.Threading.Tasks;
using Grpc.Core;
using Grpc.Net.Client;
using Grpc.Net.Client.Web;
using UnityEngine;

namespace Cheetah.Platform
{
    /// <summary>
    /// Параметры соединения кластера
    /// </summary>
    public class ClusterConnector
    {
        private readonly string url;
        private readonly int port;
        private readonly bool useSSL;
        private GrpcChannel channel;

        public ClusterConnector(string url, int port, bool useSSL)
        {
            this.url = url;
            this.port = port;
            this.useSSL = useSSL;
        }

        private void DoConnect()
        {
            var address = (useSSL ? "https" : "http") + "://" + url + ":" + port;
            channel = GrpcChannel.ForAddress(
                address, new GrpcChannelOptions
                {
                    HttpHandler = new GrpcWebHandler(new HttpClientHandler()),
                }
            );
        }

        private async Task Reconnect()
        {
            await Destroy();
            DoConnect();
        }

        public async Task Destroy()
        {
            try
            {
                if (channel != null)
                {
                    await channel.ShutdownAsync();
                }

                channel = null;
            }
            catch (Exception e)
            {
                Debug.Log(e);
            }
        }

        /// <summary>
        /// Выполнить gRPC запрос, если соединение потеряно - выполнить его еще раз
        /// </summary>
        public async Task<T> DoRequest<T>(Func<GrpcChannel, Task<T>> action)
        {
            if (channel == null)
            {
                await Reconnect();
            }

            try
            {
                return await action(channel);
            }
            catch (RpcException e)
            {
                if (e.StatusCode != StatusCode.Unavailable) throw e;
                Debug.Log(e);
                await Reconnect();
                return await action(channel);
            }
        }

        public async Task DoRequest(Func<GrpcChannel, Task> action)
        {
            if (channel == null)
            {
                await Reconnect();
            }

            try
            {
                await action(channel);
            }
            catch (RpcException e)
            {
                if (e.StatusCode != StatusCode.Unavailable) throw e;
                Debug.Log(e);
                await Reconnect();
                await action(channel);
            }
        }
    }
}