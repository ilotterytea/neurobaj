package kz.ilotterytea.neurobaj;

import com.github.twitch4j.TwitchClient;
import com.github.twitch4j.TwitchClientBuilder;
import com.github.twitch4j.helix.domain.User;
import kz.ilotterytea.neurobaj.neural.markov.MarkovChainHandler;
import kz.ilotterytea.neurobaj.storage.PropertiesLoader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.util.ArrayList;
import java.util.List;
import java.util.Properties;

/**
 * Neurobaj.
 * @author ilotterytea
 * @since 1.0
 */
public class Neurobaj {
    private final Logger LOG = LoggerFactory.getLogger(Neurobaj.class);

    private Properties properties;

    private TwitchClient twitchBot;

    private MarkovChainHandler markovChainHandler;

    private static Neurobaj instance;

    public Neurobaj() {
        instance = this;
    }

    public void init() {
        properties = new PropertiesLoader(SharedConstants.PROPERTIES_PATH);
        markovChainHandler = new MarkovChainHandler(new File("./targets"));

        if (properties.getProperty("TWITCH_CLIENTID", null) != null && properties.getProperty("TWITCH_ACCESSTOKEN", null) != null) {
            LOG.info("Initializing the Twitch module...");

            twitchBot = TwitchClientBuilder.builder()
                    .withClientId(properties.getProperty("TWITCH_CLIENTID"))
                    .withEnableChat(true)
                    .withEnableHelix(true)
                    .build();

            twitchBot.getChat().connect();

            if (markovChainHandler.getAllChains().keySet().size() > 0) {
                List<User> users = twitchBot.getHelix().getUsers(
                        properties.getProperty("TWITCH_ACCESSTOKEN"),
                        new ArrayList<>(markovChainHandler.getAllChains().keySet()),
                        null
                ).execute().getUsers();

                for (User user : users) {
                    twitchBot.getChat().joinChannel(user.getLogin());
                }
            }
        } else {
            LOG.warn("The TWITCH_CLIENTID and TWITCH_ACCESSTOKEN fields must be filled out in your config.properties if you want to use Twitch chats as your chain base source!");
        }
    }

    public void dispose() {
        if (twitchBot != null) {
            twitchBot.getChat().disconnect();
            twitchBot.close();
        }
        markovChainHandler.saveAll();
    }

    public static Neurobaj getInstance() {
        return instance;
    }

    public Properties getProperties() {
        return properties;
    }

    public TwitchClient getTwitchBot() {
        return twitchBot;
    }

    public MarkovChainHandler getMarkov() {
        return markovChainHandler;
    }
}
