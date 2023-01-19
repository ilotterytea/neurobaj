package kz.ilotterytea.neurobaj;

/**
 * Bot.
 * @author ilotterytea
 * @since 1.0
 */
public abstract class Bot {
    /**
     * Initialize the bot.
     */
    protected abstract void init();

    /**
     * Run the bot.
     */
    protected abstract void run();

    /**
     * Dispose the bot.
     */
    protected abstract void dispose();
}
