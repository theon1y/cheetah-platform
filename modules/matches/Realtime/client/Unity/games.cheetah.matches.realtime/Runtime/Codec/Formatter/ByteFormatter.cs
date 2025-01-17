using System.Runtime.CompilerServices;
using Cheetah.Matches.Realtime.Types;

namespace Cheetah.Matches.Realtime.Codec.Formatter
{
    public sealed class ByteFormatter : UnmanagedFormatter<byte>
    {
        public static readonly ByteFormatter Instance = new();


        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public override unsafe byte UncheckedRead(ref CheetahBuffer buffer)
        {
            return buffer.values[buffer.pos++];
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public override unsafe void UncheckedWrite(byte value, ref CheetahBuffer buffer)
        {
            buffer.values[buffer.size++] = value;
        }
    }
}