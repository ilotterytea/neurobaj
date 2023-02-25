package kz.ilotterytea.neurobaj.twitch;

import com.github.twitch4j.chat.events.channel.DeleteMessageEvent;
import com.github.twitch4j.chat.events.channel.IRCMessageEvent;
import com.github.twitch4j.chat.events.channel.UserBanEvent;
import kz.ilotterytea.neurobaj.Neurobaj;
import kz.ilotterytea.neurobaj.neural.markov.ChatChain;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.stream.Collectors;

/**
 * @author ilotterytea
 * @since 1.0
 */
public class MessageHandler {
    private static final Neurobaj BOT = Neurobaj.getInstance();

    public static void ircMessageEvent(IRCMessageEvent e) {
        if (
                !e.getMessage().isPresent() ||
                        !e.getMessageId().isPresent()
        ) {
            return;
        }

        String msg = e.getMessage().get();

        BOT.getMarkov().scanText(
                msg,
                e.getMessageId().get(),
                e.getChannel().getId(),
                e.getUser().getId()
        );
    }

    public static void deleteMessageEvent(DeleteMessageEvent e) {
        List<ChatChain> targetChains = BOT.getMarkov().getAllChains().getOrDefault(e.getChannel().getId(), new ArrayList<>());

        List<ChatChain> chains = targetChains
                .stream()
                .filter(c -> Objects.equals(c.getToWordAuthor().getMsgId(), e.getMsgId()))
                .collect(Collectors.toList());

        chains.addAll(
                targetChains
                        .stream()
                        .filter(c -> Objects.equals(c.getFromWordAuthor().getMsgId(), e.getMsgId()))
                        .collect(Collectors.toList())
        );

        for (ChatChain chain : chains) {
            int i = targetChains.indexOf(chain);

            if (i > -1) {
                BOT.getMarkov().getAllChains().getOrDefault(e.getChannel().getId(), new ArrayList<>()).remove(i);
            }
        }
    }

    public static void userBanEvent(UserBanEvent e) {
        List<ChatChain> targetChains = BOT.getMarkov().getAllChains().getOrDefault(e.getChannel().getId(), new ArrayList<>());

        List<ChatChain> chains = targetChains
                .stream()
                .filter(c -> Objects.equals(c.getToWordAuthor().getSenderId(), e.getUser().getId()))
                .collect(Collectors.toList());

        chains.addAll(
                targetChains
                        .stream()
                        .filter(c -> Objects.equals(c.getFromWordAuthor().getSenderId(), e.getUser().getId()))
                        .collect(Collectors.toList())
        );

        for (ChatChain chain : chains) {
            int i = targetChains.indexOf(chain);

            if (i > -1) {
                BOT.getMarkov().getAllChains().getOrDefault(e.getChannel().getId(), new ArrayList<>()).remove(i);
            }
        }
    }
}
