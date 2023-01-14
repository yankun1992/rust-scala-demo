package io.otavia.channel.mio

import io.netty5.buffer.{Buffer, BufferAllocator}

import java.nio.charset.Charset

@main def main(): Unit = {
  val allocator = BufferAllocator.offHeapPooled()

  val buffer = allocator.allocate(1024 * 8)

  val javaJNI = new JavaJNI()

  val len = JavaJNI.writeBuffer(buffer)
  val str = buffer.readCharSequence(len, Charset.forName("utf-8"))
  buffer.compact()


  val poll = Poll.createPoll()
  val server = TcpListener.bind("127.0.0.1:13265")

  poll.register(server, Interest.READABLE)

  val response =
    """HTTP/1.1 200 OK
      |Connection: keep-alive
      |Server: JNI-HTTPD/1.1
      |Content-Length: 11
      |
      |hello world""".stripMargin


  while (true) {
    poll.select()
    val events = poll.selectedKeys()

    while (events.hasNext) {
      val event = events.next()
      val socket = event.socket
      socket match
        case server: TcpListener => if (event.isReadable) {
          val client = server.accept()
          println("main: server accept")
          poll.register(client, Interest.READABLE)
        }
        case client: TcpStream => if (event.isReadable) {
          val len = client.read(buffer)
          val data = buffer.readCharSequence(len, Charset.forName("utf-8"))
          buffer.compact()
          println(s"read from client length[$len]:\n$data")
          buffer.writeCharSequence(response, Charset.forName("utf-8"))
          client.write(buffer)
        }

    }


  }
}