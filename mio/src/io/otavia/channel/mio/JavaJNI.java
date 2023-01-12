package io.otavia.channel.mio;

import io.github.otavia.jni.loader.NativeLoader;
import io.netty5.buffer.Buffer;
import io.netty5.buffer.BufferComponent;
import io.netty5.buffer.ComponentIterator;


public class JavaJNI extends NativeLoader {
    JavaJNI() {
        super("mionative");
    }

    public static native int add(int a, int b);

    public native void plus(int term);

    public static int writeBuffer(Buffer buffer) {
        ComponentIterator<?> iter = buffer.forEachComponent();
        BufferComponent component = iter.first();
        int write = JavaJNI.write0(component.writableNativeAddress(), buffer.writableBytes());
        buffer.writerOffset(buffer.writerOffset() + write);
        return write;
    }

    public static native int write0(long address, int writable);

}
