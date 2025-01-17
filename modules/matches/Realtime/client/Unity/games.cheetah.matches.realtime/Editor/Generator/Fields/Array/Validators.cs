using System.Collections.Generic;
using Cheetah.Matches.Realtime.Codec;
using Cheetah.Matches.Realtime.Editor.Generator.Fields.Array.Exceptions;

namespace Cheetah.Matches.Realtime.Editor.Generator.Fields.Array
{
    internal static class Validators
    {
        internal static void ValidateArraySizeField(ArraySizeField arraySizeFieldAttribute, HashSet<string> processedFields,
            HashSet<string> allFields)
        {
            if (processedFields.Contains(arraySizeFieldAttribute.Field)) return;

            if (allFields.Contains(arraySizeFieldAttribute.Field))
            {
                throw new IncorrectOrderArraySizeFieldException(arraySizeFieldAttribute.Field);
            }

            throw new MissingArraySizeFieldException(arraySizeFieldAttribute.Field);
        }
    }
}