package kz.ilotterytea.neurobaj.storage;

import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.io.Reader;
import java.util.Properties;

/**
 * Properties loader.
 * @author ilotterytea
 * @since 1.0
 */
public class PropertiesLoader extends Properties {
    public PropertiesLoader(String file_path) {
        super();
        load(file_path);
    }

    private void load(String file_path) {
        File file = new File(file_path);

        if (file.exists()) {
            try (Reader reader = new FileReader(file)) {
                super.load(reader);
            } catch (IOException e) {
                throw new RuntimeException(e);
            }
        }
    }
}
