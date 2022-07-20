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
		public class TestCodecArrayFieldStructureCodec:Codec<Cheetah.Matches.Relay.Tests.Runtime.Codec.Field.TestCodecArrayField.Structure>
		{
			public void Decode(ref CheetahBuffer buffer, ref Cheetah.Matches.Relay.Tests.Runtime.Codec.Field.TestCodecArrayField.Structure dest)
			{
				dest.size = ByteFormatter.Instance.Read(ref buffer);
				for(var i=0;i<dest.size;i++) {
					codec0.Decode(ref buffer, ref dest.values[i]);
				}
	
			}
	
			public void  Encode(ref Cheetah.Matches.Relay.Tests.Runtime.Codec.Field.TestCodecArrayField.Structure source, ref CheetahBuffer buffer)
			{
				ByteFormatter.Instance.Write(source.size,ref buffer);
				for(var i=0;i<source.size;i++) {
					codec0.Encode(ref source.values[i],ref buffer);
				}
	
			}
	
			private readonly Codec<Cheetah.Matches.Relay.Tests.Runtime.Codec.Field.TestCodecArrayField.Inner> codec0;
	
			private TestCodecArrayFieldStructureCodec(CodecRegistry codecRegistry)
			{
				codec0 = codecRegistry.GetCodec<Cheetah.Matches.Relay.Tests.Runtime.Codec.Field.TestCodecArrayField.Inner>();
			}
	
	
			[RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.SubsystemRegistration)]
			private static void OnRuntimeMethodLoad()
			{
				CodecRegistryBuilder.RegisterDefault(factory=>new TestCodecArrayFieldStructureCodec(factory));
			}
	
		}
	
	
}
