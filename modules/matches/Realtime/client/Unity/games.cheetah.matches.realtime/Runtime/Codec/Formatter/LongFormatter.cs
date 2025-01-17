using System.Runtime.CompilerServices;
using Cheetah.Matches.Realtime.Types;

namespace Cheetah.Matches.Realtime.Codec.Formatter
{
    public sealed class LongFormatter : UnmanagedFormatter<long>
    {
        public static readonly LongFormatter Instance = new();


        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public override long UncheckedRead(ref CheetahBuffer buffer)
        {
            return (long)ULongFormatter.StaticUncheckedRead(ref buffer);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public override void UncheckedWrite(long value, ref CheetahBuffer buffer)
        {
            ULongFormatter.StaticUncheckedWrite((ulong)value, ref buffer);
        }
    }
}