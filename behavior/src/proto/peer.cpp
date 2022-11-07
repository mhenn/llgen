#include "../msgs/GameState.pb.h"
#include "../msgs/BeaconSignal.pb.h"
#include "../msgs/BeaconSignal.pb.cc"
#include <protobuf_comm/peer.h>


using namespace protobuf_comm;
using namespace boost::placeholders;
using namespace llsf_msgs;

#define TEAM_NAME "Evolution"
#define CRYPTO_KEY "key"
#define CRYPTO_CIPHER "aes-128-cbc"

ProtobufBroadcastPeer              *peer_public_  = NULL;
ProtobufBroadcastPeer              *peer_team_    = NULL;
static boost::asio::deadline_timer *timer_ = NULL;
Team                                team_color_ = CYAN;
unsigned long                       seq_ = 0;

void
handle_timer(const boost::system::error_code &error)
{

//	if (!error) {
		boost::posix_time::ptime               now(boost::posix_time::microsec_clock::universal_time());
		std::shared_ptr<BeaconSignal>          signal(new BeaconSignal());
		Time                                  *time        = signal->mutable_time();
		boost::posix_time::time_duration const since_epoch = now - boost::posix_time::from_time_t(0);
//
//		time->set_sec(static_cast<google::protobuf::int64>(since_epoch.total_seconds()));
//		time->set_nsec(static_cast<google::protobuf::int64>(
//		  since_epoch.fractional_seconds() * (1000000000 / since_epoch.ticks_per_second())));
//
//
//		signal->set_number(1);
//		signal->set_peer_name(TEAM_NAME);
//		signal->set_team_name(TEAM_NAME);
//		signal->set_team_color(team_color_);
//		signal->set_seq(++seq_);
//		peer_team_->send(signal);
//
//		timer_->expires_at(timer_->expires_at() + boost::posix_time::milliseconds(2000));
//		timer_->async_wait(handle_timer);
//	}
}
