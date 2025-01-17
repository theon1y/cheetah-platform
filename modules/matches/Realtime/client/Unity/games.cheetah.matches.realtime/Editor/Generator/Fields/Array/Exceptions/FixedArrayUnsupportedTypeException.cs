using System;

namespace Cheetah.Matches.Realtime.Editor.Generator.Fields.Array.Exceptions
{
    public class FixedArrayUnsupportedTypeException : Exception
    {
        public FixedArrayUnsupportedTypeException(string elementTypeName) : base("Unsupported type " + elementTypeName + " for fixed array field.")
        {
        }
    }
}