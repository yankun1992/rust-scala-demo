package io.otavia.channel.mio

import io.netty5.buffer.{Buffer, BufferComponent, ComponentIterator}

class TcpStream(val raw: Long) extends MioSocket {
  override val socketId: Int = MioSocket.getNextId

  def read(buffer: Buffer): Int = {
    val iterator = buffer.forEachComponent[BufferComponent with ComponentIterator.Next]()
    val component = iterator.firstWritable()
    val write = read0(raw, component.writableNativeAddress, buffer.writableBytes)
    buffer.writerOffset(buffer.writerOffset + write)
    iterator.close()
    write
  }

  def write(buffer: Buffer): Int = {
    val iterator = buffer.forEachComponent[BufferComponent with ComponentIterator.Next]()
    val component = iterator.firstReadable()
    val read = write0(raw, component.readableNativeAddress(), buffer.readableBytes())
    buffer.readerOffset(buffer.readerOffset() + read)
    iterator.close()
    read
  }


  @native def read0(raw: Long, buffer: Long, writable: Int): Int

  @native def write0(raw: Long, buffer: Long, readable: Int): Int
}
