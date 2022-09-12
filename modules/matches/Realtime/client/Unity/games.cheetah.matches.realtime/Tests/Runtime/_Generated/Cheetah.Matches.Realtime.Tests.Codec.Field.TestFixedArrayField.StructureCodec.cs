using Cheetah.Matches.Realtime.Codec;
using Cheetah.Matches.Realtime.Codec.Formatter;
using Cheetah.Matches.Realtime.Types;
using UnityEngine;
using Cheetah.Matches.Realtime.Tests.Codec.Field;

// ReSharper disable once CheckNamespace
namespace Cheetah_Matches_Realtime_Tests_Codec_Field
{
		// warning warning warning warning warning
		// Code generated by Cheetah relay codec generator - DO NOT EDIT
		// warning warning warning warning warning
		public class TestFixedArrayFieldStructureCodec:Codec<Cheetah.Matches.Realtime.Tests.Codec.Field.TestFixedArrayField.Structure>
		{
			public void Decode(ref CheetahBuffer buffer, ref Cheetah.Matches.Realtime.Tests.Codec.Field.TestFixedArrayField.Structure dest)
			{
				dest.size = ByteFormatter.Instance.Read(ref buffer);
				unsafe {
					fixed (System.Byte* data = dest.byteArray) {
						ByteFormatter.Instance.ReadFixedArray(ref buffer, data, dest.size,0);
					}
				}
	
				unsafe {
					fixed (System.Byte* data = dest.arrayWithoutSizeVariable) {
						ByteFormatter.Instance.ReadFixedArray(ref buffer, data, 2,0);
					}
				}
	
				unsafe {
					fixed (System.UInt32* data = dest.variableSizeArray) {
						VariableSizeUIntFormatter.Instance.ReadFixedArray(ref buffer, data, dest.size,0);
					}
				}
	
			}
	
			public void  Encode(in Cheetah.Matches.Realtime.Tests.Codec.Field.TestFixedArrayField.Structure source, ref CheetahBuffer buffer)
			{
				ByteFormatter.Instance.Write(source.size,ref buffer);
				unsafe {
					fixed (System.Byte* data = source.byteArray) {
						ByteFormatter.Instance.WriteFixedArray(data,source.size,0,ref buffer);
					}
				}
	
				unsafe {
					fixed (System.Byte* data = source.arrayWithoutSizeVariable) {
						ByteFormatter.Instance.WriteFixedArray(data,2,0,ref buffer);
					}
				}
	
				unsafe {
					fixed (System.UInt32* data = source.variableSizeArray) {
						VariableSizeUIntFormatter.Instance.WriteFixedArray(data,source.size,0,ref buffer);
					}
				}
	
			}
	
	
			[RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.SubsystemRegistration)]
			private static void OnRuntimeMethodLoad()
			{
				CodecRegistryBuilder.RegisterDefault(factory=>new TestFixedArrayFieldStructureCodec());
			}
	
		}
	
	
}