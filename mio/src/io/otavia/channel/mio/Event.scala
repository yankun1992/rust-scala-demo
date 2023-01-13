package io.otavia.channel.mio

class Event(val raw: Long, val poll: Poll) {
  def socket: MioSocket = {
    val socketId = token0(raw)
    poll.getSocket(socketId)
  }

  def isReadable: Boolean = isReadable0(raw)

  def isWritable: Boolean = isWritable0(raw)

  @native def token0(raw: Long): Int

  @native def isReadable0(raw: Long): Boolean

  @native def isWritable0(raw: Long): Boolean
}

