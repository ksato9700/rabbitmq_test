import pika
import os
import sys


def callback(ch, method, properties, body):
    print(f' [x] Received {body}')


def main():
    connection = pika.BlockingConnection(
        pika.ConnectionParameters('localhost'))
    with connection.channel() as channel:
        channel.basic_consume(
            queue='hello',
            auto_ack=True,
            on_message_callback=callback
        )
        channel.start_consuming()


if __name__ == '__main__':
    try:
        main()
    except KeyboardInterrupt:
        print('interrupted')
        try:
            sys.exit(0)
        except SystemExit:
            os._exit(0)
