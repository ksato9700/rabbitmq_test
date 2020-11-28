const amqplib = require('amqplib');
const q = 'hello';

async function main() {

    const conn = await amqplib.connect('amqp://localhost');
    const ch = await conn.createChannel();
    const queue_options = {
        durable: false,
    }
    await ch.assertQueue(q, queue_options);
    const sent = await ch.sendToQueue(q, Buffer.from('Hello World'));
    console.log('sent', sent);
    process.exit(0)
}
main();
