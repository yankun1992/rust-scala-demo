package io.otavia.channel.mio;

import io.github.otavia.jni.loader.NativeLoader;
import io.netty5.buffer.Buffer;
import io.netty5.buffer.BufferComponent;


public class JavaJNI extends NativeLoader {
    JavaJNI() {
        super("mionative");
    }

    public static native int add(int a, int b);

    public native void plus(int term);

    public static int writeBuffer(Buffer buffer) {
        BufferComponent component = buffer.forEachComponent().first();
        int write = JavaJNI.write0(component.writableNativeAddress(), buffer.writableBytes());
        buffer.writerOffset(buffer.writerOffset() + write);
        return write;
    }

    public static native int write0(long address, int writable);

}
