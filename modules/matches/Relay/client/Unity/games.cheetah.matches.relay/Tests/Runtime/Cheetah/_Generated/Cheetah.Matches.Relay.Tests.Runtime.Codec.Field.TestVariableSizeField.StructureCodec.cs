using Cheetah.Matches.Relay.Codec;
using Cheetah.Matches.Relay.Codec.Formatter;
using Cheetah.Matches.Relay.Types;
using UnityEngine;
using Cheetah.Matches.Relay.Tests.Runtime.Codec.Field;

// ReSharper disable once CheckNamespace
namespace Cheetah.Matches.Relay.Tests.Runtime.Codec.Field
{
		// warning warning warning warning warning
		// Code generated by Cheetah relay codec generator - DO NOT EDIT
		// warning warning warning warning warning
		public class TestVariableSizeFieldStructureCodec:Codec<Cheetah.Matches.Relay.Tests.Runtime.Codec.Field.TestVariableSizeField.Structure>
		{
			public void Decode(ref CheetahBuffer buffer, ref Cheetah.Matches.Relay.Tests.Runtime.Codec.Field.TestVariableSizeField.Structure dest)
			{
				dest.uintValue = VariableSizeUIntFormatter.Instance.Read(ref buffer);
				dest.ulongValue = VariableSizeULongFormatter.Instance.Read(ref buffer);
				dest.intValue = VariableSizeIntFormatter.Instance.Read(ref buffer);
				dest.longValue = VariableSizeLongFormatter.Instance.Read(ref buffer);
			}
	
			public void  Encode(ref Cheetah.Matches.Relay.Tests.Runtime.Codec.Field.TestVariableSizeField.Structure source, ref CheetahBuffer buffer)
			{
				VariableSizeUIntFormatter.Instance.Write(source.uintValue,ref buffer);
				VariableSizeULongFormatter.Instance.Write(source.ulongValue,ref buffer);
				VariableSizeIntFormatter.Instance.Write(source.intValue,ref buffer);
				VariableSizeLongFormatter.Instance.Write(source.longValue,ref buffer);
			}
	
	
			[RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.SubsystemRegistration)]
			private static void OnRuntimeMethodLoad()
			{
				CodecRegistryBuilder.RegisterDefault(factory=>new TestVariableSizeFieldStructureCodec());
			}
	
		}
	
	
}
