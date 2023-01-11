package io.otavia.channel.mio

import io.github.otavia.jni.loader.NativeLoader

class Events(val capacity: Int = 1024, val belong: Poll) extends NativeLoader("mionative") with Iterator[Event] {

  val raw: Long = Events.openEvents(capacity)

  /** Mapping to rust Events Iter, update by [[Poll.select]] */
  private var iterRaw: Option[Long] = None

  /** Mapping to rust `Box<Option<&Event>>`, update by [[hasNext]] */
  private var nextCache: Option[Long] = None

  def setIterRaw(raw: Long): Unit = iterRaw = Some(raw)

  override def hasNext: Boolean = if (iterRaw.nonEmpty) {
    nextCache match
      case Some(_) => true
      case None =>
        val optRaw = next0(iterRaw.get)
        if (optIsEmpty0(optRaw)) false else {
          nextCache = Some(optRaw)
          true
        }
  } else false

  override def next(): Event = nextCache match
    case Some(value) =>
      nextCache = None
      new Event(value, belong)
    case None =>
      if (hasNext)
        next()
      else
        throw new NoSuchElementException("")

  @native private def next0(raw: Long): Long

  @native private def optIsEmpty0(raw: Long): Boolean

}


object Events extends NativeLoader("mionative") {

  @native def openEvents(capacity: Int): Long

  def hello(): Unit = println("hello")


}