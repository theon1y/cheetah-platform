using System;
using UnityEngine;

namespace CheetahRelay
{
    /**
     * Отбражение логов с клиента в Unity консоле
     */
    public class LoggerGateway
    {
        public static void Init()
        {
            LoggerExternals.InitLogger();
        }

        private static void ShowLog(LogLevel level, string log)
        {
            switch (level)
            {
                case LogLevel.Info:
                    Debug.Log(log);
                    break;
                case LogLevel.Warn:
                    Debug.LogWarning(log);
                    break;
                case LogLevel.Error:
                    Debug.LogError(log);
                    break;
                default:
                    throw new ArgumentOutOfRangeException(nameof(level), level, null);
            }
        }

        public static void CollectLogs()
        {
            LoggerExternals.CollectLogs(ShowLog);
        }
    }
}