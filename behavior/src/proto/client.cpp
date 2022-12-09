#include <protobuf_comm/client.h>
#include "../../proto_msgs/GameState.pb.h"
#include "../../proto_msgs/GameInfo.pb.h"
#include "../../proto_msgs/RobotInfo.pb.h"
#include "../../proto_msgs/VersionInfo.pb.h"
#include "../../proto_msgs/MachineCommands.pb.h"
#include <string.h>

#include "./peer.cpp"

#include <unistd.h>

using namespace llsf_msgs;
using namespace protobuf_comm;

ProtobufStreamClient *client_;
static boost::asio::io_service io_service_;
SetTeamName  *msg_team_cyan_ = NULL;
SetGamePhase *msg_phase_ = NULL;
SetGameState *msg_state_ = NULL;
bool switched = false;


void
quit(int exitcode = 0, const char *errmsg = NULL)
{
	if (errmsg)
		fprintf(stderr, "%s\n", errmsg);
	io_service_.stop();
}


void
send_team(){
    msg_team_cyan_ = new SetTeamName();
    msg_team_cyan_->set_team_name(TEAM_NAME);
    msg_team_cyan_->set_team_color(CYAN);

  printf("Sending cyan team: %s\n", msg_team_cyan_->team_name().c_str());
            client_->send(*msg_team_cyan_);
}


void
send_add_base(){
    printf("Adding base\n");
	MachineAddBase add_base;
	add_base.set_machine_name("C-BS");
	client_->send(add_base);

}

void pick_base(){

}

void place_base(){
}

void pick_product(){
}

void deliver_product(){
}

void
send_game_state(std::string state, std::string phase){

        GameState::Phase p;
		GameState::Phase_Parse(state, &p);

        GameState::State s;
		GameState::State_Parse(phase, &s);

        msg_phase_ = new llsf_msgs::SetGamePhase();
		msg_phase_->set_phase(p);
        msg_state_ = new llsf_msgs::SetGameState();
		msg_state_->set_state(s);
                std::cout << "PHASE: " << state << std::endl;
				client_->send(*msg_state_);
                std::cout << "STATE: " << phase << std::endl;
				client_->send(*msg_phase_);
}


bool second = false;
bool third = false;
bool fourth = false;
bool fifth = false;

void
client_msg(uint16_t comp_id, uint16_t msg_type, std::shared_ptr<google::protobuf::Message> msg)
{
	std::shared_ptr<VersionInfo> v;
        if ((v = std::dynamic_pointer_cast<VersionInfo>(msg))) {
			// connected, send what we came for
            send_team();
            send_game_state("SETUP", "RUNNING");
	}

    std::shared_ptr<GameState> gs;
	if ((gs = std::dynamic_pointer_cast<GameState>(msg)) && !switched) {
            usleep(1000000);
            send_game_state("PRODUCTION", "RUNNING");
            switched = true;
            second = true;
    } else if(second){
        third = true;
        second = false;
        usleep(2000000);
        send_retrieve_cap();
    }
    else if(third){
        fourth = true;
        third = false;
        usleep(2000000);
        send_get_base();
    }
    else if(fourth){
        fourth = false;
        fifth= true;
        usleep(2000000);
        send_mount_cap();
    }
    else if(fifth){
        fifth = false;
        usleep(2000000);
        send_deliver();
    }

}


void
handle_disconnected(const boost::system::error_code &ec)
{
		fprintf(stderr, "Failed to connect: %s\n", ec.message().c_str());
		quit(1);
}

void
signal_handler(const boost::system::error_code &error, int signum)
{
	if (!error)
		quit();
}


void setup_client(){
    client_ = new ProtobufStreamClient();

    MessageRegister &message_register = client_->message_register();
	message_register.add_message_type<GameState>();
	message_register.add_message_type<RobotInfo>();
	message_register.add_message_type<VersionInfo>();

	client_->signal_received().connect(client_msg);
	client_->signal_disconnected().connect(handle_disconnected);
    //client_->async_connect("172.18.0.22", 4444);
    client_->async_connect("localhost", 4444);

    boost::asio::signal_set signals(io_service_, SIGINT, SIGTERM);

	// Start an asynchronous wait for one of the signals to occur.
	signals.async_wait(signal_handler);
}
