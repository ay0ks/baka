import proto.message as message
import socket

msg = message.Message()
msg.author = "ay0ks"
msg.content = "message {all} :test"

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect(("127.0.0.1", 20030))
s.sendall(msg.SerializeToString())

print(s.recv(100))