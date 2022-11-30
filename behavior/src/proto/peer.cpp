#ifndef HEADER_INCLUDED_
#define HEADER_INCLUDED_
//#include "BeaconSignal.pb.h"
#include "../../proto_msgs/BeaconSignal.pb.h"
#include "../../proto_msgs/MachineInfo.pb.h"
#include "../../proto_msgs/OrderInfo.pb.h"
#include "../../proto_msgs/GameState.pb.h"
#include "../../proto_msgs/MachineCommands.pb.h"
#include "../../proto_msgs/ExplorationInfo.pb.h"
#include "../../proto_msgs/MachineReport.pb.h"
#include "../../proto_msgs/RingInfo.pb.h"
#include "../../proto_msgs/RobotInfo.pb.h"
#include "../../proto_msgs/VersionInfo.pb.h"
#include <protobuf_comm/peer.h>

#include <string.h>

using namespace protobuf_comm;
using namespace boost::placeholders;
using namespace llsf_msgs;

#define TEAM_NAME "Evolution"
#define CRYPTO_KEY "key"
#define CRYPTO_CIPHER "aes-128-cbc"

ProtobufBroadcastPeer *peer_public_ = NULL;
ProtobufBroadcastPeer *peer_team_ = NULL;
Team team_color_ = CYAN;
unsigned long seq_ = 0;

void handle_beacon() {

  boost::posix_time::ptime now(
      boost::posix_time::microsec_clock::universal_time());
  std::shared_ptr<BeaconSignal> signal(new BeaconSignal());
  Time *time = signal->mutable_time();
  boost::posix_time::time_duration const since_epoch =
      now - boost::posix_time::from_time_t(0);

    time->set_sec(static_cast<google::protobuf::int64>(since_epoch.total_seconds()));
    time->set_nsec(static_cast<google::protobuf::int64>(
      since_epoch.fractional_seconds() * (1000000000 /
       since_epoch.ticks_per_second())));

  		signal->set_number(1);
  		signal->set_peer_name(TEAM_NAME);
  		signal->set_team_name(TEAM_NAME);
  		signal->set_team_color(team_color_);
  		signal->set_seq(++seq_);
  		peer_team_->send(signal);
}

void
handle_recv_error(boost::asio::ip::udp::endpoint &endpoint, std::string msg)
{
	printf("Receive error from %s:%u: %s\n",
	       endpoint.address().to_string().c_str(),
	       endpoint.port(),
	       msg.c_str());
}

void
handle_send_error(std::string msg)
{
	printf("Send error: %s\n", msg.c_str());
}

void
handle_message(boost::asio::ip::udp::endpoint            &sender,
               uint16_t                                   component_id,
               uint16_t                                   msg_type,
               std::shared_ptr<google::protobuf::Message> msg)
{
    std::shared_ptr<MachineInfo> mi;
	if ((mi = std::dynamic_pointer_cast<MachineInfo>(msg))) {
		printf("MachineInfo received:\n");
		for (int i = 0; i < mi->machines_size(); ++i) {
			const Machine &m = mi->machines(i);
			printf("  %s, state: %s\n", m.name().c_str(), m.state().c_str());
		}
	}

    std::shared_ptr<GameState> gs;
	if ((gs = std::dynamic_pointer_cast<GameState>(msg))) {
		int hour = gs->game_time().sec() / 3600;
		int min  = (gs->game_time().sec() - hour * 3600) / 60;
		int sec  = gs->game_time().sec() - hour * 3600 - min * 60;

		printf("GameState received:  %02i:%02i:%02i.%02ld  %s %s  %u:%u points, %s vs. %s\n",
		       hour,
		       min,
		       sec,
		       gs->game_time().nsec() / 1000000,
		       llsf_msgs::GameState::Phase_Name(gs->phase()).c_str(),
		       llsf_msgs::GameState::State_Name(gs->state()).c_str(),
		       gs->points_cyan(),
		       gs->points_magenta(),
		       gs->team_cyan().c_str(),
		       gs->team_magenta().c_str());

		if (TEAM_NAME == gs->team_cyan() || TEAM_NAME == gs->team_magenta()) {
			if (TEAM_NAME == gs->team_cyan() && team_color_ != CYAN) {
				printf("WARNING: sending as magenta, but our team is announced as cyan by refbox!\n");
			} else if (TEAM_NAME == gs->team_magenta() && team_color_ != MAGENTA) {
				printf("WARNING: sending as cyan, but our team is announced as magenta by refbox!\n");
			}
		}
	}
}

void set_game_phase(){}

void setup_proto(){

//    string host = "172.18.0.22";
    std::string host = "localhost";
    peer_public_ = new ProtobufBroadcastPeer(host, 4445, 4444);
    MessageRegister &message_register = peer_public_->message_register();
    message_register.add_message_type<BeaconSignal>();
	message_register.add_message_type<OrderInfo>();
	message_register.add_message_type<GameState>();
	message_register.add_message_type<VersionInfo>();
	message_register.add_message_type<ExplorationInfo>();
	message_register.add_message_type<MachineInfo>();
	message_register.add_message_type<MachineReportInfo>();
	message_register.add_message_type<RobotInfo>();
	message_register.add_message_type<RingInfo>();

    peer_team_ = new ProtobufBroadcastPeer(host, 4446,4441, &message_register);

    peer_team_->signal_received().connect(handle_message);
	peer_team_->signal_recv_error().connect(handle_recv_error);
	peer_team_->signal_send_error().connect(handle_send_error);

    peer_public_->signal_received().connect(handle_message);
	peer_public_->signal_recv_error().connect(handle_recv_error);
	peer_public_->signal_send_error().connect(handle_send_error);

}

#endif
