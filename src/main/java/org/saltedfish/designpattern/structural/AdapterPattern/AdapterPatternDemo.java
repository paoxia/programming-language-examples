package org.saltedfish.designpattern.structural.AdapterPattern;

public class AdapterPatternDemo {
    public static void main(String[] args) {
        AudioPlayer audioPlayer = new AudioPlayer();

        audioPlayer.play("mp3", "dongfengpo");
        audioPlayer.play("mp4", "touwenziD");

    }
}
