#ifndef HEADER_INCLUDED_
#define HEADER_INCLUDED_
// #include "BeaconSignal.pb.h"
#include "../../proto_msgs/BeaconSignal.pb.h"
#include "../../proto_msgs/ExplorationInfo.pb.h"
#include "../../proto_msgs/GameState.pb.h"
#include "../../proto_msgs/MachineCommands.pb.h"
#include "../../proto_msgs/MachineInstructions.pb.h"
#include "../../proto_msgs/MachineInfo.pb.h"
#include "../../proto_msgs/MachineReport.pb.h"
#include "../../proto_msgs/OrderInfo.pb.h"
#include "../../proto_msgs/RingInfo.pb.h"
#include "../../proto_msgs/RobotInfo.pb.h"
#include "../../proto_msgs/VersionInfo.pb.h"
#include <protobuf_comm/peer.h>
#include "../utils/order.cpp"


using namespace protobuf_comm;
using namespace boost::placeholders;
using namespace llsf_msgs;

#define TEAM_NAME "Carologistics"
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

  time->set_sec(
      static_cast<google::protobuf::int64>(since_epoch.total_seconds()));
  time->set_nsec(static_cast<google::protobuf::int64>(
      since_epoch.fractional_seconds() *
      (1000000000 / since_epoch.ticks_per_second())));

  signal->set_number(1);
  signal->set_peer_name(TEAM_NAME);
  signal->set_team_name(TEAM_NAME);
  signal->set_team_color(team_color_);
  signal->set_seq(++seq_);
  peer_team_->send(signal);
}


//#prepare cap
//./rcll-prepare-machine Carologistics C-CS1 RETRIEVE_CAP & sleep 10 ; kill $!
//
//#get base
//./rcll-prepare-machine Carologistics C-BS INPUT BASE_BLACK & sleep 10 ; kill $!
//
//#mount cap
//./rcll-prepare-machine Carologistics C-CS1 MOUNT_CAP & sleep 10 ; kill $!
//
//#deliver
//./rcll-prepare-machine Carologistics C-DS 3 & sleep 10 ; kill $!


//void send_prepare_machine(std::string machine_name, std::string machine_type,
//         std::string side, std::string base, std::string operation){
void send_prepare_machine(){
        CSOp op;
        MachineSide bs_side;
        BaseColor bs_color;

        std::string machine_name = "C-BS";
        std::string machine_type = "BS";
        std::string side = "INPUT";
        std::string base = "BASE_RED";

        //llsf_msgs::CSOp_Parse(operation, op);
        llsf_msgs::MachineSide_Parse(side, &bs_side);
		llsf_msgs::BaseColor_Parse(base,&bs_color);

    printf("Announcing machine type\n");
			llsf_msgs::PrepareMachine prep;
			prep.set_team_color(team_color_);
			prep.set_machine(machine_name);
			auto duration = std::chrono::system_clock::now().time_since_epoch();
			auto millis   = std::chrono::duration_cast<std::chrono::seconds>(duration).count();
			prep.set_sent_at(millis);
			if (machine_type == "BS") {
				llsf_msgs::PrepareInstructionBS *prep_bs = prep.mutable_instruction_bs();
				prep_bs->set_side(bs_side);
				prep_bs->set_color(bs_color);
				printf("Set BS side %s  color %s\n",
				      MachineSide_Name(bs_side).c_str(),
				      BaseColor_Name(bs_color).c_str());
            }
            else if (machine_type == "CS") {
                            PrepareInstructionCS *prep_cs = prep.mutable_instruction_cs();
                            prep_cs->set_operation(op);
            }
            peer_team_->send(prep);
}


void send_machine_state(std::string name, MachineState state) {
  printf("Sending State\n");
  SetMachineState set_state;
  set_state.set_machine_name(name);
  set_state.set_state(state);
  peer_team_->send(set_state);
}


void handle_recv_error(boost::asio::ip::udp::endpoint &endpoint,
                       std::string msg) {
  printf("Receive error from %s:%u: %s\n",
         endpoint.address().to_string().c_str(), endpoint.port(), msg.c_str());
}

void handle_send_error(std::string msg) {
  printf("Send error: %s\n", msg.c_str());
}

static inline std::string str_join(const std::vector<std::string> &v,
                                   char delim = '/') {
  std::string rv;
  for (size_t i = 0; i < v.size(); ++i) {
    if (i > 0)
      rv += delim;
    rv += v[i];
  }
  return rv;
}



void handle_message(boost::asio::ip::udp::endpoint &sender,
                    uint16_t component_id, uint16_t msg_type,
                    std::shared_ptr<google::protobuf::Message> msg) {
  std::shared_ptr<MachineInfo> mi;
  if ((mi = std::dynamic_pointer_cast<MachineInfo>(msg))) {
    printf("MachineInfo received:\n");
    for (int i = 0; i < mi->machines_size(); ++i) {
      const Machine &m = mi->machines(i);

      printf("  %s, state: %s, color: %s\n", m.name().c_str(),
             m.state().c_str(), "");
    }
  }

  std::shared_ptr<GameState> gs;
  if ((gs = std::dynamic_pointer_cast<GameState>(msg))) {
    int hour = gs->game_time().sec() / 3600;
    int min = (gs->game_time().sec() - hour * 3600) / 60;
    int sec = gs->game_time().sec() - hour * 3600 - min * 60;

    printf("GameState received:  %02i:%02i:%02i.%02ld  %s %s  %u:%u points, %s "
           "vs. %s\n",
           hour, min, sec, gs->game_time().nsec() / 1000000,
           llsf_msgs::GameState::Phase_Name(gs->phase()).c_str(),
           llsf_msgs::GameState::State_Name(gs->state()).c_str(),
           gs->points_cyan(), gs->points_magenta(), gs->team_cyan().c_str(),
           gs->team_magenta().c_str());

    if (TEAM_NAME == gs->team_cyan() || TEAM_NAME == gs->team_magenta()) {
      if (TEAM_NAME == gs->team_cyan() && team_color_ != CYAN) {
        printf("WARNING: sending as magenta, but our team is announced as cyan "
               "by refbox!\n");
      } else if (TEAM_NAME == gs->team_magenta() && team_color_ != MAGENTA) {
        printf("WARNING: sending as cyan, but our team is announced as magenta "
               "by refbox!\n");
      }
    }
  }




  std::shared_ptr<OrderInfo> oi;
  if ((oi = std::dynamic_pointer_cast<OrderInfo>(msg))) {
    printf("Order Info received:\n");
    for (int i = 0; i < oi->orders_size(); ++i) {
      const llsf_msgs::Order &o = oi->orders(i);
      unsigned int begin_min = o.delivery_period_begin() / 60;
      unsigned int begin_sec = o.delivery_period_begin() - begin_min * 60;
      unsigned int end_min = o.delivery_period_end() / 60;
      unsigned int end_sec = o.delivery_period_end() - end_min * 60;

      orderInstance->base=llsf_msgs::BaseColor_Name(o.base_color());
      orderInstance->cap=llsf_msgs::CapColor_Name(o.cap_color());

      std::vector<std::string> rings;
      std::string lel = "";
      for (int j = 0; j < o.ring_colors_size(); ++j)
        rings.push_back(llsf_msgs::RingColor_Name(o.ring_colors(j)));

      printf("  %u (%s): %u%u/%u of %s|%s|%s at gate %u \n", o.id(),
             llsf_msgs::Order::Complexity_Name(o.complexity()).c_str(),
             o.quantity_delivered_cyan(), o.quantity_delivered_magenta(),
             o.quantity_requested(),
             llsf_msgs::BaseColor_Name(o.base_color()).c_str(),
             str_join(rings, '-').c_str(),
             llsf_msgs::CapColor_Name(o.cap_color()).c_str(),
             o.delivery_gate());
    }
  }
}

void setup_proto() {

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

  peer_team_ = new ProtobufBroadcastPeer(host, 4446, 4441, &message_register);

  peer_team_->signal_received().connect(handle_message);
  peer_team_->signal_recv_error().connect(handle_recv_error);
  peer_team_->signal_send_error().connect(handle_send_error);

  peer_public_->signal_received().connect(handle_message);
  peer_public_->signal_recv_error().connect(handle_recv_error);
  peer_public_->signal_send_error().connect(handle_send_error);

}

#endif
