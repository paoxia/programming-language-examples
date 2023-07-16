package org.saltedfish.designpattern.structural.ProxyPattern;

public class ProxyImage implements Image{

    //通过ProxyImage去代理RealImage
    private RealImage realImage;

    private String fileName;

    public ProxyImage(String fileName) {
        this.fileName = fileName;
    }

    @Override
    public void display() {
        if (realImage == null) {
            realImage = new RealImage(fileName);
        }
        realImage.display();
    }
}
