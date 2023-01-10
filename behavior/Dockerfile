FROM ubuntu:focal

RUN apt update

ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Europe/Berlin
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
RUN apt install -y protobuf-compiler libprotobuf-* git libzmq3-dev libboost-coroutine-dev libncurses5-dev libncursesw5-dev
RUN apt install -y cmake build-essential libssl-dev libprotoc-dev

WORKDIR /proto_comm
RUN git clone https://github.com/fawkesrobotics/protobuf_comm.git /proto_comm
RUN cmake . && make -j8 && make install

RUN mkdir build
RUN git clone https://github.com/BehaviorTree/BehaviorTree.CPP.git /bt
WORKDIR /bt
RUN git checkout 3.8.1
WORKDIR /bt/build
RUN cmake .. && make -j8 && make install

RUN cp -r /usr/local/include/behaviortree_cpp_v3/ /usr/local/include/behaviortree_cpp/
RUN dir /usr/local/include/

COPY . /behavior


WORKDIR /behavior/proto_msgs
RUN make clean && rm -rf CMakeFiles cmake_install.cmake CMakeCache.txt
RUN cmake .. && make -j8
WORKDIR /behavior/build
RUN make clean && rm -rf *
RUN cmake .. && make -j8

