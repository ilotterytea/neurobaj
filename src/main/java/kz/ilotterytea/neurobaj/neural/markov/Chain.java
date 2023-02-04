package kz.ilotterytea.neurobaj.neural.markov;

import java.util.ArrayList;
import java.util.Arrays;

/**
 * Chain for Markov.
 * @author ilotterytea
 * @since 1.0
 */
public class Chain {
    /** The word with which the chain begins. */
    private final String fromWord;
    /** The word with which the chain ends. */
    private String toWord;

    public Chain(
            String fromWord,
            String toWord
    ) {
        this.fromWord = fromWord;
        this.toWord = toWord;
    }

    /** Tokenize the text into an array of chains. */
    public static ArrayList<Chain> tokenize(String text) {
        ArrayList<Chain> list = new ArrayList<>();
        ArrayList<String> s = new ArrayList<>(Arrays.asList(text.split(" ")));
        String previousWord = "\\x02";

        for (String w : s) {
            list.add(new Chain(previousWord, w));
            previousWord = w;
        }

        list.add(new Chain(previousWord, "\\x03"));
        return list;
    }

    public String getFromWord() { return fromWord; }
    public String getToWord() { return toWord; }
    public void setToWord(String toWord) { this.toWord = toWord; }
}
