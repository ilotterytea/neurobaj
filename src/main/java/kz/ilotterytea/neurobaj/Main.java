package kz.ilotterytea.neurobaj;

import io.micronaut.runtime.Micronaut;

/**
 * @author ilotterytea
 * @since 1.0
 */
public class Main {
    public static void main(String[] args) {
        System.out.println("Hello world!");

        Micronaut.build(args)
                .classes(Main.class)
                .start();
    }
}