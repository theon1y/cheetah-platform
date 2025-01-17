using System.IO;
using UnityEngine;

namespace Cheetah.Platform.Editor.Configuration
{
    public static class ConfigurationUtils
    {
        /// <summary>
        /// Путь до файлов, необходимых для запуска сервиса в на внешнем хостинге
        /// </summary>
        /// <param name="serverName">Символическое имя сервера</param>
        /// <returns></returns>
        public static string GetPathToConfigDirectory(string serverName)
        {
            return Path.GetFullPath(Path.Combine(Application.dataPath, "../", ConfigurationSettings.GetOrCreateSettings().Directory, serverName));
        }

        public static void InitConfigDirectoryIfNotExists(string packageAssetPath, string serverName)
        {
            var destinationPath = GetPathToConfigDirectory(serverName);
            if (Directory.Exists(destinationPath)) return;
            var sourcePath = Path.GetFullPath(packageAssetPath + "/ConfigTemplates/");
            if (!Directory.Exists(sourcePath)) return;

            Directory.CreateDirectory(destinationPath);
            CopyDirectory(sourcePath, destinationPath);
        }

        private static void CopyDirectory(string sourceDir, string destinationDir)
        {
            var dir = new DirectoryInfo(sourceDir);
            if (!dir.Exists)
                throw new DirectoryNotFoundException($"Source directory not found: {dir.FullName}");

            var dirs = dir.GetDirectories();
            Directory.CreateDirectory(destinationDir);
            foreach (var file in dir.GetFiles())
            {
                var targetFilePath = Path.Combine(destinationDir, file.Name);
                if (file.Name.EndsWith(".meta")) continue;
                file.CopyTo(targetFilePath);
            }

            foreach (var subDir in dirs)
            {
                var newDestinationDir = Path.Combine(destinationDir, subDir.Name);
                CopyDirectory(subDir.FullName, newDestinationDir);
            }
        }
    }
}