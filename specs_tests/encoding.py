import asyncio
import websockets
import json

# for a u32: 42 and according to the scale encoding docs,
# the encoded data should be:
EXPECTED_RESPONSE_U32 = '2a000000'

# for more info go: https://docs.substrate.io/reference/scale-codec/

class RpcMessage:
    def __init__(self, method, param_name, param_value):
        self.method = method
        self.param_name = param_name
        self.param_value = param_value
    
    def set_param(self, param_name, param_value):
        self.param_name = param_name
        self.param_value = param_value
    
    def to_dict(self):
        return {
            "jsonrpc": "2.0",
            "method": self.method,
            "params": {
                "test": {
                    self.param_name: self.param_value
                }
            },
            "id": 1
        }

def make_payload(variant_name, value):
    message = RpcMessage("scale_encode", variant_name, value)
    return json.dumps(message.to_dict())


# pub enum ScaleMsg {
# 	U32(u32),
# 	I64(i32),
# 	F64(f64),
# 	Str(String),
# 	Vec(Vec<u8>),
# 	Tuple((u16, String)),
# }
def prepare_testing_data():
    return [make_payload("U32", 42), make_payload("Str", "Zondax"), make_payload("I64", -1), make_payload("Vec", [4, 8, 15, 16, 23, 42])]

async def send_messages(websocket, messages):
    responses = []
    for message in messages:
        await websocket.send(message)
        print(f"> Sent: {message}")

        response = await websocket.recv()
        print(f"< Received: {response}")
        responses.append(response)

    return responses


async def main():
    uri = "ws://127.0.0.1:9944"
    async with websockets.connect(uri) as websocket:

        messages = prepare_testing_data()
        responses = await send_messages(websocket, messages)  # collect the responses

        # Parse the first response
        response_data = json.loads(responses[0])
        result = response_data.get('result', '')  # Adjust this line based on the actual response format

        # Compare the result with the constant, ensure
        # they are the same
        # TODO: we also need to check for the other responses
        assert result == EXPECTED_RESPONSE_U32, f"Unexpected result: {result}"

if __name__ == "__main__":
    asyncio.run(main())


