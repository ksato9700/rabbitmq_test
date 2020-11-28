const amqplib = require('amqplib');
const q = 'hello';

async function main() {

    const conn = await amqplib.connect('amqp://localhost');
    const ch = await conn.createChannel();
    const queue_options = {
        durable: false,
    }
    await ch.assertQueue(q, queue_options);
    await ch.consume(q, (msg) => {
        console.log(` [x] Received "${msg.content.toString()}"`)
    });

}
main();
