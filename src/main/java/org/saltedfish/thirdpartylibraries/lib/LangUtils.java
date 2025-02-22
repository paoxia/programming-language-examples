package org.saltedfish.thirdpartylibraries.lib;

import java.util.ArrayList;
import java.util.List;
import org.apache.commons.collections4.CollectionUtils;
import org.apache.commons.lang3.BooleanUtils;
import org.apache.commons.lang3.StringUtils;

/**
 * LangUtils
 */
public class LangUtils {

    public static void main(String[] args) {
        System.out.println(StringUtils.isEmpty(StringUtils.EMPTY));

        List<String> list = new ArrayList<>();
        System.out.println(CollectionUtils.isEmpty(list));

        Boolean b = false;
        System.out.println(BooleanUtils.isFalse(b));
    }
}
