package kz.ilotterytea.neurobaj.neural.markov;

/**
 * An extended version of the chain, but only here with chain authors.
 * Designed for Twitch and can also help clean up unwanted chains from their specific authors.
 * @author ilotterytea
 * @since 1.0
 */
public class ChatChain extends Chain {
    private final ChainAuthor fromWordAuthor;
    private ChainAuthor toWordAuthor;

    public ChatChain(
            String fromWord,
            String toWord,
            ChainAuthor fromWordAuthor,
            ChainAuthor toWordAuthor
    ) {
        super(fromWord, toWord);
        this.fromWordAuthor = fromWordAuthor;
        this.toWordAuthor = toWordAuthor;
    }

    public ChainAuthor getFromWordAuthor() { return fromWordAuthor; }
    public ChainAuthor getToWordAuthor() { return toWordAuthor; }
    public void setToWordAuthor(ChainAuthor author) { this.toWordAuthor = author; }
}
