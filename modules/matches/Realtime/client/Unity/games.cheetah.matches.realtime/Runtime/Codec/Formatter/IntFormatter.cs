using System.Runtime.CompilerServices;
using Cheetah.Matches.Realtime.Types;

namespace Cheetah.Matches.Realtime.Codec.Formatter
{
    public class IntFormatter : UnmanagedFormatter<int>
    {
        public static readonly IntFormatter Instance = new();


        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public override int UncheckedRead(ref CheetahBuffer buffer)
        {
            return (int)UIntFormatter.StaticUncheckedRead(ref buffer);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public override void UncheckedWrite(int value, ref CheetahBuffer buffer)
        {
            UIntFormatter.StaticUncheckedWrite((uint)value, ref buffer);
        }
    }
}