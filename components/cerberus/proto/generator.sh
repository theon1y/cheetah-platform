#!/bin/bash
protoc --plugin=protoc-gen-grpc=/usr/local/Cellar/grpc/1.33.2_3/bin/grpc_csharp_plugin --grpc_out=../clients/Unity/Packages/games.cheetah.unity.cerberus/Runtime/ --csharp_out=../clients/Unity/Packages/games.cheetah.unity.cerberus/Runtime/ service.external.proto