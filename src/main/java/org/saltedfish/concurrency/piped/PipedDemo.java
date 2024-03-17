package org.saltedfish.concurrency.piped;

import java.io.IOException;
import java.io.PipedReader;
import java.io.PipedWriter;

public class PipedDemo {

    class ReaderThread implements Runnable {
        private PipedReader reader;

        public ReaderThread(PipedReader reader) {
            this.reader = reader;
        }

        @Override
        public void run() {
            System.out.println("Reader reader");
            int receive = 0;

            try {
                while ((receive = reader.read()) != -1) {
                    System.out.println((char) receive);
                }
            } catch (IOException e) {
                throw new RuntimeException(e);
            }

        }
    }

    class WriterThread implements Runnable {
        private PipedWriter writer;

        public WriterThread(PipedWriter writer) {
            this.writer = writer;
        }

        @Override
        public void run() {
            System.out.println("Writer writer");
            try {
                writer.write("test");
            } catch (IOException e) {
                throw new RuntimeException(e);
            } finally {
                try {
                    writer.close();
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
        }

    }


    public static void main(String[] args) throws IOException {
        PipedWriter writer = new PipedWriter();
        PipedReader reader = new PipedReader();

        writer.connect(reader);

        Thread thread1 = new Thread(new PipedDemo().new ReaderThread(reader));
        Thread thread2 = new Thread(new PipedDemo().new WriterThread(writer));

        thread1.start();
        thread2.start();

    }
}
