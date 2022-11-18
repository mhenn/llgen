#ifndef HEADER_INCLUDED_
#define HEADER_INCLUDED_
//#include "BeaconSignal.pb.h"
#include "../../proto_msgs/BeaconSignal.pb.h"
#include <protobuf_comm/peer.h>

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

    std::cout << "Bruh: "  << std::endl;

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
	std::shared_ptr<BeaconSignal> b;
		       b->number(),
		       b->team_name().c_str(),
		       b->peer_name().c_str(),
		       b->seq();
}


void setup_proto(){

    peer_public_ = new ProtobufBroadcastPeer("localhost", 4445,4444);

    MessageRegister &message_register = peer_public_->message_register();
	message_register.add_message_type<BeaconSignal>();

    peer_team_ = new ProtobufBroadcastPeer("localhost", 4446,4441);

    peer_public_->signal_received().connect(handle_message);
	peer_public_->signal_recv_error().connect(handle_recv_error);
	peer_public_->signal_send_error().connect(handle_send_error);

}

#endif
