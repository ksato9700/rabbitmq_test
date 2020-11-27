import pika
def main():
    connection = pika.BlockingConnection(pika.ConnectionParameters('localhost'))
    with connection.channel() as channel:
        channel.queue_declare(queue='hello')
        channel.basic_publish(
            exchange='',
            routing_key='hello',
            body='Hello World!')
        print(' [x] Sent "Hello World!"')

if __name__ == '__main__':
    main()
