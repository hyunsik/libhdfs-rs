#!/bin/bash

if [ -z $1 ];
then
  echo "usage: ./update_proto.sh \$HADOOP_SRC_DIR"
  exit 1
fi

PROTO_FILES="ClientDatanodeProtocol.proto
ClientNamenodeProtocol.proto
IpcConnectionContext.proto
ProtobufRpcEngine.proto
RpcHeader.proto
Security.proto
datatransfer.proto
hdfs.proto
acl.proto 
xattr.proto 
encryption.proto 
inotify.proto"

for each_file in $PROTO_FILES; do
  cp `find $1 -name $each_file` proto
done
