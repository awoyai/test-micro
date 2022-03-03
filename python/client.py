import struct
import user_pb2
import user_pb2_grpc
import grpc


def run():
        channel = grpc.insecure_channel('127.0.0.1:3000')
        stud = user_pb2_grpc.UserStub(channel)
        req = user_pb2.LoginRequest(name='fff', password='123456')
        resp = stud.Login(req)
        print(resp)

if __name__=="__main__":
    run()