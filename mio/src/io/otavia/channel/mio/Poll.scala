package io.otavia.channel.mio

import io.github.otavia.jni.loader.NativeLoader

import scala.collection.mutable

class Poll(val raw: Long) {

  private val events = new Events(belong = this)

  val sockets = mutable.HashMap.empty[Int, MioSocket]

  def getSocket(id: Int): MioSocket = sockets(id)

  def register(socket: MioSocket, interest: Interest): Unit = {
    socket match
      case listener: TcpListener => register0(raw, socket.raw, socket.socketId, interest.value, 0)
      case stream: TcpStream => register0(raw, socket.raw, socket.socketId, interest.value, 1)

    sockets.put(socket.socketId, socket)

  }

  def select(): Unit = select(0, 0)

  def select(secs: Long, nanos: Long): Unit = events.setIterRaw(select0(raw, events.raw, secs, nanos))

  def selectedKeys(): Events = events

  @native private def register0(poll: Long, socket: Long, id: Int, interest: Int, tp: Int): Unit

  @native private def select0(poll: Long, events: Long, secs: Long, nanos: Long): Long


}

object Poll extends NativeLoader("mionative") {

  def createPoll(): Poll = new Poll(openPoll())

  @native def openPoll(): Long


}
