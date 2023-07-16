package org.saltedfish.io;

import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;

/**
 * @author SaltedFish
 * @date 2021/2/21
 */
public class ByteStreamsDemo {
    public static void main(String[] args) throws IOException {
        FileInputStream in = null;
        FileOutputStream out = null;

        try {
            in = new FileInputStream("src/main/java/org/saltedfish/io/input.txt");
            out = new FileOutputStream("src/main/java/org/saltedfish/io/ByteStreamsDemoOutput.txt");
            int c;

            while ((c = in.read()) != -1) {
                out.write(c);
            }

        } finally {
            if (in != null) {
                in.close();
            }
            if (out != null) {
                out.close();
            }
        }
    }
}
