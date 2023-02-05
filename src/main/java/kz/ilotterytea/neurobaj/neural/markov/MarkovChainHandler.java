package kz.ilotterytea.neurobaj.neural.markov;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.google.gson.reflect.TypeToken;
import kz.ilotterytea.neurobaj.SharedConstants;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.*;
import java.util.*;

/**
 * @author ilotterytea
 * @since 1.0
 */
public class MarkovChainHandler {
    private final Logger log = LoggerFactory.getLogger(MarkovChainHandler.class);
    private final Map<String, ArrayList<ChatChain>> chains;
    private final File file;

    public MarkovChainHandler(File file) {
        this.chains = new HashMap<>();
        this.file = file;

        if (file.isDirectory()) {
            for (File f : Objects.requireNonNull(file.listFiles())) {
                processFile(f);
            }
        } else {
            processFile(file);
        }
    }

    public String generateText(String text) {
        ArrayList<String> s = new ArrayList<>(Arrays.asList(text.trim().split(" ")));
        StringBuilder sb = new StringBuilder();

        int chainIndex = 0;
        int usedChainsStats = 0;
        int usedTargetsStats = 0;

        for (String w : s) {
            if (chainIndex > chains.size() - 1) {
                chainIndex = 0;
            }

            ArrayList<ChatChain> chatChains = chains.get(
                    new ArrayList<>(chains.keySet()).get(chainIndex)
            );

            Chain first = chatChains
                    .stream()
                    .filter(c -> Objects.equals(c.getFromWord(), w))
                    .findFirst()
                    .orElse(null);

            if (
                    first == null ||
                            Objects.equals(first.getToWord(), "\\x03") ||
                            Objects.equals(first.getToWord(), "")
            ) {
                chainIndex++;
                continue;
            }

            usedTargetsStats++;

            Chain nextChain = null;

            while (true) {
                Chain chain;

                if (nextChain == null) {
                    sb.append(first.getFromWord()).append(" ");

                    chain = chatChains
                            .stream()
                            .filter(c -> Objects.equals(c.getFromWord(), first.getToWord()))
                            .findFirst()
                            .orElse(null);

                    if (chain == null) {
                        break;
                    }

                    nextChain = chain;
                } else {
                    sb.append(nextChain.getFromWord()).append(" ");

                    Chain finalNextChain = nextChain;
                    chain = chatChains
                            .stream()
                            .filter(c -> Objects.equals(c.getFromWord(), finalNextChain.getToWord()))
                            .findFirst()
                            .orElse(null);

                    if (chain == null) {
                        break;
                    }

                    nextChain = chain;
                }

                usedChainsStats++;
            }

            chainIndex++;
        }

        log.debug("Generated a new text: \"" + sb + "\" (Total " + usedChainsStats + " chains of " + usedTargetsStats + " channels were used)");
        return sb.toString();
    }

    public void scanText(
            String text,
            String msgId,
            String channelId,
            String senderId
    ) {
        ArrayList<Chain> chains1 = Chain.tokenize(text);
        ArrayList<ChatChain> targetChains = chains.getOrDefault(channelId, new ArrayList<>());

        for (Chain chain : chains1) {
            ChatChain record = targetChains
                    .stream()
                    .filter(c -> Objects.equals(c.getFromWord(), chain.getFromWord()))
                    .findFirst().orElse(null);

            if (record != null) {
                record.setToWord(chain.getToWord());
                record.setToWordAuthor(new ChainAuthor(msgId, senderId));
            } else {
                targetChains.add(new ChatChain(
                        chain.getFromWord(),
                        chain.getToWord(),
                        new ChainAuthor(msgId, senderId),
                        new ChainAuthor(msgId, senderId)
                ));
            }
        }

        chains.put(channelId, targetChains);
        log.debug("Scanned " + chains1.size() + " chain(s)!");
    }

    public Map<String, ArrayList<ChatChain>> getAllChains() { return chains; }

    private void processFile(File f) {
        if (f.isDirectory()) {
            log.warn("File \"" + f.getAbsolutePath() + "\" is a directory. Aborted processing the file!");
            return;
        }

        if (!f.exists()) {
            log.warn("File \"" + file.getAbsolutePath() + "\" not exist. Aborted processing the file!");
            return;
        }

        try (Reader reader = new FileReader(f)) {
            ArrayList<ChatChain> records = new Gson().fromJson(reader, new TypeToken<ArrayList<ChatChain>>(){}.getType());
            String targetId = f.getName().split("\\.")[0];

            if (records != null) {
                chains.put(targetId, records);
                log.info("Markov chain status: +" + records.size() + " loaded! Total: " + chains.size() + " chains.");
            }
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public void saveAll() {
        for (String key : chains.keySet()) {
            save(new File(SharedConstants.TARGETS_DIR_PATH + "/" + key + ".json"), key);
        }
    }

    public void save(File file, String targetId) {
        try (Writer writer = new FileWriter(file)) {
            writer.write(new GsonBuilder().setPrettyPrinting().create().toJson(chains.get(targetId)));
            log.debug("Saved the Target ID " + targetId + "!");
        } catch (IOException e) {
            throw  new RuntimeException(e);
        }
    }
}
