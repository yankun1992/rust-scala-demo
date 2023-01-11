package io.otavia.channel.mio

import io.netty5.buffer.Buffer

class TcpStream(val raw: Long) extends MioSocket {
  override val socketId: Int = MioSocket.getNextId

  def read(buffer: Buffer): Int = ???


  @native def read0(raw: Long, buffer: Long): Int
}
