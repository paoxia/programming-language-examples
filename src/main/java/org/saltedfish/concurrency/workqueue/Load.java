package org.saltedfish.concurrency.workqueue;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@Builder
@AllArgsConstructor
@NoArgsConstructor
public class Load {
    /**
     * 结果
     */
    private int num;
}
