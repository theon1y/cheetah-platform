using System.Runtime.InteropServices;

namespace CheetahRelay
{
    public static class CheetahStructure
    {
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public delegate void Listener(ref CheetahCommandMeta meta, ref CheetahObjectId objectId, ushort fieldId, ref CheetahBuffer data);

        /// <summary>
        /// Установить обработчик серверных команд для текущего клиента
        /// </summary>
        /// <param name="listener"></param>
        /// <returns>false - клиент не найден</returns>
        [DllImport(Const.Library, CallingConvention = CallingConvention.Cdecl, EntryPoint = "set_structure_listener")]
        public static extern bool SetListener(Listener listener);


        /// <summary>
        /// Установить значение
        /// </summary>
        /// <param name="objectId"></param>
        /// <param name="fieldId"></param>
        /// <param name="value"></param>
        /// <returns>false - клиент не найден</returns>
        [DllImport(Const.Library, CallingConvention = CallingConvention.Cdecl, EntryPoint = "set_structure")]
        public static extern bool Set(ref CheetahObjectId objectId, ushort fieldId, ref CheetahBuffer data);
    }
}