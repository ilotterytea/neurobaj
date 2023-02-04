package kz.ilotterytea.neurobaj.neural.markov;

/**
 * Author Chain. Designed for Twitch IRC messages.
 * @author ilotterytea
 * @since 1.0
 */
public class ChainAuthor {
    /** Message ID. */
    private final String msgId;
    /** Sender ID. */
    private final String senderId;

    public ChainAuthor(
            String msgId,
            String senderId
    ) {
        this.msgId = msgId;
        this.senderId = senderId;
    }

    public String getMsgId() { return msgId; }
    public String getSenderId() { return senderId; }
}
