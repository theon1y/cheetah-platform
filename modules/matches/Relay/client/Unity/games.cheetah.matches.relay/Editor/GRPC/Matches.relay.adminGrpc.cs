// <auto-generated>
//     Generated by the protocol buffer compiler.  DO NOT EDIT!
//     source: matches.relay.admin.proto
// </auto-generated>
#pragma warning disable 0414, 1591
#region Designer generated code

using grpc = global::Grpc.Core;

namespace Cheetah.Matches.Relay.Editor.GRPC {
  /// <summary>
  ///*
  ///Общие команды для получения информации и управления сервером
  /// </summary>
  public static partial class Relay
  {
    static readonly string __ServiceName = "cheetah.matches.relay.admin.Relay";

    static readonly grpc::Marshaller<global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsRequest> __Marshaller_cheetah_matches_relay_admin_GetRoomsRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsResponse> __Marshaller_cheetah_matches_relay_admin_GetRoomsResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsResponse.Parser.ParseFrom);

    static readonly grpc::Method<global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsRequest, global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsResponse> __Method_GetRooms = new grpc::Method<global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsRequest, global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "GetRooms",
        __Marshaller_cheetah_matches_relay_admin_GetRoomsRequest,
        __Marshaller_cheetah_matches_relay_admin_GetRoomsResponse);

    /// <summary>Service descriptor</summary>
    public static global::Google.Protobuf.Reflection.ServiceDescriptor Descriptor
    {
      get { return global::Cheetah.Matches.Relay.Editor.GRPC.MatchesRelayAdminReflection.Descriptor.Services[0]; }
    }

    /// <summary>Base class for server-side implementations of Relay</summary>
    [grpc::BindServiceMethod(typeof(Relay), "BindService")]
    public abstract partial class RelayBase
    {
      /// <summary>
      ///*
      ///Получить список комнат
      /// </summary>
      /// <param name="request">The request received from the client.</param>
      /// <param name="context">The context of the server-side call handler being invoked.</param>
      /// <returns>The response to send back to the client (wrapped by a task).</returns>
      public virtual global::System.Threading.Tasks.Task<global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsResponse> GetRooms(global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

    }

    /// <summary>Client for Relay</summary>
    public partial class RelayClient : grpc::ClientBase<RelayClient>
    {
      /// <summary>Creates a new client for Relay</summary>
      /// <param name="channel">The channel to use to make remote calls.</param>
      public RelayClient(grpc::ChannelBase channel) : base(channel)
      {
      }
      /// <summary>Creates a new client for Relay that uses a custom <c>CallInvoker</c>.</summary>
      /// <param name="callInvoker">The callInvoker to use to make remote calls.</param>
      public RelayClient(grpc::CallInvoker callInvoker) : base(callInvoker)
      {
      }
      /// <summary>Protected parameterless constructor to allow creation of test doubles.</summary>
      protected RelayClient() : base()
      {
      }
      /// <summary>Protected constructor to allow creation of configured clients.</summary>
      /// <param name="configuration">The client configuration.</param>
      protected RelayClient(ClientBaseConfiguration configuration) : base(configuration)
      {
      }

      /// <summary>
      ///*
      ///Получить список комнат
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The response received from the server.</returns>
      public virtual global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsResponse GetRooms(global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return GetRooms(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      ///*
      ///Получить список комнат
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The response received from the server.</returns>
      public virtual global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsResponse GetRooms(global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_GetRooms, null, options, request);
      }
      /// <summary>
      ///*
      ///Получить список комнат
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The call object.</returns>
      public virtual grpc::AsyncUnaryCall<global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsResponse> GetRoomsAsync(global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return GetRoomsAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      ///*
      ///Получить список комнат
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The call object.</returns>
      public virtual grpc::AsyncUnaryCall<global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsResponse> GetRoomsAsync(global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_GetRooms, null, options, request);
      }
      /// <summary>Creates a new instance of client from given <c>ClientBaseConfiguration</c>.</summary>
      protected override RelayClient NewInstance(ClientBaseConfiguration configuration)
      {
        return new RelayClient(configuration);
      }
    }

    /// <summary>Creates service definition that can be registered with a server</summary>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static grpc::ServerServiceDefinition BindService(RelayBase serviceImpl)
    {
      return grpc::ServerServiceDefinition.CreateBuilder()
          .AddMethod(__Method_GetRooms, serviceImpl.GetRooms).Build();
    }

    /// <summary>Register service method with a service binder with or without implementation. Useful when customizing the  service binding logic.
    /// Note: this method is part of an experimental API that can change or be removed without any prior notice.</summary>
    /// <param name="serviceBinder">Service methods will be bound by calling <c>AddMethod</c> on this object.</param>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static void BindService(grpc::ServiceBinderBase serviceBinder, RelayBase serviceImpl)
    {
      serviceBinder.AddMethod(__Method_GetRooms, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsRequest, global::Cheetah.Matches.Relay.Editor.GRPC.GetRoomsResponse>(serviceImpl.GetRooms));
    }

  }
  /// <summary>
  ///*
  ///Получения состояния комнаты для отладки
  /// </summary>
  public static partial class Dump
  {
    static readonly string __ServiceName = "cheetah.matches.relay.admin.Dump";

    static readonly grpc::Marshaller<global::Cheetah.Matches.Relay.Editor.GRPC.DumpRequest> __Marshaller_cheetah_matches_relay_admin_DumpRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Cheetah.Matches.Relay.Editor.GRPC.DumpRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Cheetah.Matches.Relay.Editor.GRPC.DumpResponse> __Marshaller_cheetah_matches_relay_admin_DumpResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Cheetah.Matches.Relay.Editor.GRPC.DumpResponse.Parser.ParseFrom);

    static readonly grpc::Method<global::Cheetah.Matches.Relay.Editor.GRPC.DumpRequest, global::Cheetah.Matches.Relay.Editor.GRPC.DumpResponse> __Method_Dump = new grpc::Method<global::Cheetah.Matches.Relay.Editor.GRPC.DumpRequest, global::Cheetah.Matches.Relay.Editor.GRPC.DumpResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "Dump",
        __Marshaller_cheetah_matches_relay_admin_DumpRequest,
        __Marshaller_cheetah_matches_relay_admin_DumpResponse);

    /// <summary>Service descriptor</summary>
    public static global::Google.Protobuf.Reflection.ServiceDescriptor Descriptor
    {
      get { return global::Cheetah.Matches.Relay.Editor.GRPC.MatchesRelayAdminReflection.Descriptor.Services[1]; }
    }

    /// <summary>Base class for server-side implementations of Dump</summary>
    [grpc::BindServiceMethod(typeof(Dump), "BindService")]
    public abstract partial class DumpBase
    {
      public virtual global::System.Threading.Tasks.Task<global::Cheetah.Matches.Relay.Editor.GRPC.DumpResponse> Dump(global::Cheetah.Matches.Relay.Editor.GRPC.DumpRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

    }

    /// <summary>Client for Dump</summary>
    public partial class DumpClient : grpc::ClientBase<DumpClient>
    {
      /// <summary>Creates a new client for Dump</summary>
      /// <param name="channel">The channel to use to make remote calls.</param>
      public DumpClient(grpc::ChannelBase channel) : base(channel)
      {
      }
      /// <summary>Creates a new client for Dump that uses a custom <c>CallInvoker</c>.</summary>
      /// <param name="callInvoker">The callInvoker to use to make remote calls.</param>
      public DumpClient(grpc::CallInvoker callInvoker) : base(callInvoker)
      {
      }
      /// <summary>Protected parameterless constructor to allow creation of test doubles.</summary>
      protected DumpClient() : base()
      {
      }
      /// <summary>Protected constructor to allow creation of configured clients.</summary>
      /// <param name="configuration">The client configuration.</param>
      protected DumpClient(ClientBaseConfiguration configuration) : base(configuration)
      {
      }

      public virtual global::Cheetah.Matches.Relay.Editor.GRPC.DumpResponse Dump(global::Cheetah.Matches.Relay.Editor.GRPC.DumpRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Dump(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Cheetah.Matches.Relay.Editor.GRPC.DumpResponse Dump(global::Cheetah.Matches.Relay.Editor.GRPC.DumpRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_Dump, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Cheetah.Matches.Relay.Editor.GRPC.DumpResponse> DumpAsync(global::Cheetah.Matches.Relay.Editor.GRPC.DumpRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return DumpAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Cheetah.Matches.Relay.Editor.GRPC.DumpResponse> DumpAsync(global::Cheetah.Matches.Relay.Editor.GRPC.DumpRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_Dump, null, options, request);
      }
      /// <summary>Creates a new instance of client from given <c>ClientBaseConfiguration</c>.</summary>
      protected override DumpClient NewInstance(ClientBaseConfiguration configuration)
      {
        return new DumpClient(configuration);
      }
    }

    /// <summary>Creates service definition that can be registered with a server</summary>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static grpc::ServerServiceDefinition BindService(DumpBase serviceImpl)
    {
      return grpc::ServerServiceDefinition.CreateBuilder()
          .AddMethod(__Method_Dump, serviceImpl.Dump).Build();
    }

    /// <summary>Register service method with a service binder with or without implementation. Useful when customizing the  service binding logic.
    /// Note: this method is part of an experimental API that can change or be removed without any prior notice.</summary>
    /// <param name="serviceBinder">Service methods will be bound by calling <c>AddMethod</c> on this object.</param>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static void BindService(grpc::ServiceBinderBase serviceBinder, DumpBase serviceImpl)
    {
      serviceBinder.AddMethod(__Method_Dump, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Cheetah.Matches.Relay.Editor.GRPC.DumpRequest, global::Cheetah.Matches.Relay.Editor.GRPC.DumpResponse>(serviceImpl.Dump));
    }

  }
  /// <summary>
  ///*
  ///Сервис получения сетевых команд для отладки
  /// </summary>
  public static partial class CommandTracer
  {
    static readonly string __ServiceName = "cheetah.matches.relay.admin.CommandTracer";

    static readonly grpc::Marshaller<global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionRequest> __Marshaller_cheetah_matches_relay_admin_CreateSessionRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionResponse> __Marshaller_cheetah_matches_relay_admin_CreateSessionResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionResponse.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterRequest> __Marshaller_cheetah_matches_relay_admin_SetFilterRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterResponse> __Marshaller_cheetah_matches_relay_admin_SetFilterResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterResponse.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsRequest> __Marshaller_cheetah_matches_relay_admin_GetCommandsRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsResponse> __Marshaller_cheetah_matches_relay_admin_GetCommandsResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsResponse.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionRequest> __Marshaller_cheetah_matches_relay_admin_CloseSessionRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionResponse> __Marshaller_cheetah_matches_relay_admin_CloseSessionResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionResponse.Parser.ParseFrom);

    static readonly grpc::Method<global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionRequest, global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionResponse> __Method_CreateSession = new grpc::Method<global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionRequest, global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "CreateSession",
        __Marshaller_cheetah_matches_relay_admin_CreateSessionRequest,
        __Marshaller_cheetah_matches_relay_admin_CreateSessionResponse);

    static readonly grpc::Method<global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterRequest, global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterResponse> __Method_SetFilter = new grpc::Method<global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterRequest, global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "SetFilter",
        __Marshaller_cheetah_matches_relay_admin_SetFilterRequest,
        __Marshaller_cheetah_matches_relay_admin_SetFilterResponse);

    static readonly grpc::Method<global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsRequest, global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsResponse> __Method_GetCommands = new grpc::Method<global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsRequest, global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "GetCommands",
        __Marshaller_cheetah_matches_relay_admin_GetCommandsRequest,
        __Marshaller_cheetah_matches_relay_admin_GetCommandsResponse);

    static readonly grpc::Method<global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionRequest, global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionResponse> __Method_CloseSession = new grpc::Method<global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionRequest, global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "CloseSession",
        __Marshaller_cheetah_matches_relay_admin_CloseSessionRequest,
        __Marshaller_cheetah_matches_relay_admin_CloseSessionResponse);

    /// <summary>Service descriptor</summary>
    public static global::Google.Protobuf.Reflection.ServiceDescriptor Descriptor
    {
      get { return global::Cheetah.Matches.Relay.Editor.GRPC.MatchesRelayAdminReflection.Descriptor.Services[2]; }
    }

    /// <summary>Base class for server-side implementations of CommandTracer</summary>
    [grpc::BindServiceMethod(typeof(CommandTracer), "BindService")]
    public abstract partial class CommandTracerBase
    {
      /// <summary>
      ///*
      ///Создать сессию
      /// </summary>
      /// <param name="request">The request received from the client.</param>
      /// <param name="context">The context of the server-side call handler being invoked.</param>
      /// <returns>The response to send back to the client (wrapped by a task).</returns>
      public virtual global::System.Threading.Tasks.Task<global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionResponse> CreateSession(global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      /// <summary>
      ///*
      ///Установить фильтр для фильтрации команд
      /// </summary>
      /// <param name="request">The request received from the client.</param>
      /// <param name="context">The context of the server-side call handler being invoked.</param>
      /// <returns>The response to send back to the client (wrapped by a task).</returns>
      public virtual global::System.Threading.Tasks.Task<global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterResponse> SetFilter(global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      /// <summary>
      ///*
      ///Получить отфильтрованные команды
      /// </summary>
      /// <param name="request">The request received from the client.</param>
      /// <param name="context">The context of the server-side call handler being invoked.</param>
      /// <returns>The response to send back to the client (wrapped by a task).</returns>
      public virtual global::System.Threading.Tasks.Task<global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsResponse> GetCommands(global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      public virtual global::System.Threading.Tasks.Task<global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionResponse> CloseSession(global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

    }

    /// <summary>Client for CommandTracer</summary>
    public partial class CommandTracerClient : grpc::ClientBase<CommandTracerClient>
    {
      /// <summary>Creates a new client for CommandTracer</summary>
      /// <param name="channel">The channel to use to make remote calls.</param>
      public CommandTracerClient(grpc::ChannelBase channel) : base(channel)
      {
      }
      /// <summary>Creates a new client for CommandTracer that uses a custom <c>CallInvoker</c>.</summary>
      /// <param name="callInvoker">The callInvoker to use to make remote calls.</param>
      public CommandTracerClient(grpc::CallInvoker callInvoker) : base(callInvoker)
      {
      }
      /// <summary>Protected parameterless constructor to allow creation of test doubles.</summary>
      protected CommandTracerClient() : base()
      {
      }
      /// <summary>Protected constructor to allow creation of configured clients.</summary>
      /// <param name="configuration">The client configuration.</param>
      protected CommandTracerClient(ClientBaseConfiguration configuration) : base(configuration)
      {
      }

      /// <summary>
      ///*
      ///Создать сессию
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The response received from the server.</returns>
      public virtual global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionResponse CreateSession(global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return CreateSession(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      ///*
      ///Создать сессию
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The response received from the server.</returns>
      public virtual global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionResponse CreateSession(global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_CreateSession, null, options, request);
      }
      /// <summary>
      ///*
      ///Создать сессию
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The call object.</returns>
      public virtual grpc::AsyncUnaryCall<global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionResponse> CreateSessionAsync(global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return CreateSessionAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      ///*
      ///Создать сессию
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The call object.</returns>
      public virtual grpc::AsyncUnaryCall<global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionResponse> CreateSessionAsync(global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_CreateSession, null, options, request);
      }
      /// <summary>
      ///*
      ///Установить фильтр для фильтрации команд
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The response received from the server.</returns>
      public virtual global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterResponse SetFilter(global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return SetFilter(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      ///*
      ///Установить фильтр для фильтрации команд
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The response received from the server.</returns>
      public virtual global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterResponse SetFilter(global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_SetFilter, null, options, request);
      }
      /// <summary>
      ///*
      ///Установить фильтр для фильтрации команд
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The call object.</returns>
      public virtual grpc::AsyncUnaryCall<global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterResponse> SetFilterAsync(global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return SetFilterAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      ///*
      ///Установить фильтр для фильтрации команд
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The call object.</returns>
      public virtual grpc::AsyncUnaryCall<global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterResponse> SetFilterAsync(global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_SetFilter, null, options, request);
      }
      /// <summary>
      ///*
      ///Получить отфильтрованные команды
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The response received from the server.</returns>
      public virtual global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsResponse GetCommands(global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return GetCommands(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      ///*
      ///Получить отфильтрованные команды
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The response received from the server.</returns>
      public virtual global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsResponse GetCommands(global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_GetCommands, null, options, request);
      }
      /// <summary>
      ///*
      ///Получить отфильтрованные команды
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The call object.</returns>
      public virtual grpc::AsyncUnaryCall<global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsResponse> GetCommandsAsync(global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return GetCommandsAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      ///*
      ///Получить отфильтрованные команды
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The call object.</returns>
      public virtual grpc::AsyncUnaryCall<global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsResponse> GetCommandsAsync(global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_GetCommands, null, options, request);
      }
      public virtual global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionResponse CloseSession(global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return CloseSession(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionResponse CloseSession(global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_CloseSession, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionResponse> CloseSessionAsync(global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return CloseSessionAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionResponse> CloseSessionAsync(global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_CloseSession, null, options, request);
      }
      /// <summary>Creates a new instance of client from given <c>ClientBaseConfiguration</c>.</summary>
      protected override CommandTracerClient NewInstance(ClientBaseConfiguration configuration)
      {
        return new CommandTracerClient(configuration);
      }
    }

    /// <summary>Creates service definition that can be registered with a server</summary>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static grpc::ServerServiceDefinition BindService(CommandTracerBase serviceImpl)
    {
      return grpc::ServerServiceDefinition.CreateBuilder()
          .AddMethod(__Method_CreateSession, serviceImpl.CreateSession)
          .AddMethod(__Method_SetFilter, serviceImpl.SetFilter)
          .AddMethod(__Method_GetCommands, serviceImpl.GetCommands)
          .AddMethod(__Method_CloseSession, serviceImpl.CloseSession).Build();
    }

    /// <summary>Register service method with a service binder with or without implementation. Useful when customizing the  service binding logic.
    /// Note: this method is part of an experimental API that can change or be removed without any prior notice.</summary>
    /// <param name="serviceBinder">Service methods will be bound by calling <c>AddMethod</c> on this object.</param>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static void BindService(grpc::ServiceBinderBase serviceBinder, CommandTracerBase serviceImpl)
    {
      serviceBinder.AddMethod(__Method_CreateSession, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionRequest, global::Cheetah.Matches.Relay.Editor.GRPC.CreateSessionResponse>(serviceImpl.CreateSession));
      serviceBinder.AddMethod(__Method_SetFilter, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterRequest, global::Cheetah.Matches.Relay.Editor.GRPC.SetFilterResponse>(serviceImpl.SetFilter));
      serviceBinder.AddMethod(__Method_GetCommands, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsRequest, global::Cheetah.Matches.Relay.Editor.GRPC.GetCommandsResponse>(serviceImpl.GetCommands));
      serviceBinder.AddMethod(__Method_CloseSession, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionRequest, global::Cheetah.Matches.Relay.Editor.GRPC.CloseSessionResponse>(serviceImpl.CloseSession));
    }

  }
}
#endregion
