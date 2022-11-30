cmake_minimum_required(VERSION 3.10.2)

project(msgs)

#FILE(GLOB HDRS "generated/*.h")
#FILE(GLOB SRCS "generated/*.cc")
#ADD_LIBRARY(proto_msgs ${HDRS} ${SRCS})
## ${PROTOBUF_LIBRARIES} should be defined by FIND_PACKAGE(Protobuf REQUIRED)
#TARGET_LINK_LIBRARIES(proto_msgs ${PROTOBUF_LIBRARIES})
#

find_package(Protobuf REQUIRED)

list(APPEND protos
    Time.proto
    Team.proto
    Pose2D.proto
    BeaconSignal.proto
    GameInfo.proto
    GameState.proto
)

add_library(msgs)
protobuf_generate(TARGET ${PROJECT_NAME} PROTOS Time.proto Pose2D.proto
                    Team.proto BeaconSignal.proto  GameState.proto MachineCommands.proto
                    MachineInfo.proto ProductColor.proto Zone.proto MachineInstructions.proto
                    MachineDescription.proto RobotInfo.proto VersionInfo.proto GameInfo.proto
                    OrderInfo.proto ExplorationInfo.proto MachineReport.proto RingInfo.proto)
target_link_libraries(${PROJECT_NAME} protobuf)
