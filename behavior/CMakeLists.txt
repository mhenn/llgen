cmake_minimum_required(VERSION 3.10.2)
project(BTRCLL)

include_directories(./src)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(behaviortree_cpp)
find_package(Protobuf REQUIRED)
find_package(protobuf_comm)
find_package(Boost REQUIRED COMPONENTS filesystem)


include(GNUInstallDirs)
include(CMakePackageConfigHelpers)

ADD_SUBDIRECTORY("proto_msgs")

add_executable(${PROJECT_NAME} "./src/main.cpp"  )
#protobuf_generate(TARGET ${PROJECT_NAME} PROTOS Time.proto Pose2D.proto Team.proto BeaconSignal.proto)
target_link_libraries(${PROJECT_NAME} protobuf  protobuf_comm Boost::filesystem   BT::behaviortree_cpp msgs)


