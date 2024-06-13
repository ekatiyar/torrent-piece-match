# Torrent Piece Matcher

## Motivation
This project was initiated as an exploration into Rust programming and the mechanics of torrent files. The main objective was to enable the reuse of torrent pieces across different versions of files, particularly relevant in the context of Linux operating system distributions. By facilitating the reuse of unchanged pieces between closely related file versions, such as Linux OS ISO images, the aim was to reduce the bandwidth requirements when updating to newer versions. This approach could prove beneficial in scenarios where many systems need to be updated to newer software versions that are distributed via torrents.

## Result
Through experimentation with Linux Mint ISO torrents, it was observed that there was minimal piece reuse between different types and versions of OS torrents. This outcome was expected, given that modifications to data content or size within a torrent could alter both the piece data and the boundaries at which each piece begins. Specifically analyzing Linux Mint ISOs revealed that typically only a few pieces at the beginning of the torrent matched, indicating limited potential for significant piece reuse.

#### Output for comparing Mint ISOs
```
> ./torrent-piece-match linuxmint-21.2-cinnamon-64bit-edge.iso.torrent linuxmint-21.3-cinnamon-64bit-edge.iso.torrent
Matching Piece: 5 : "bf696c94d84e7ddb84134b075b03a62e6b158a2a"
Matching Piece: 6 : "a9e2cd8f06aaab9d5f955d58e0f5d5b51d4b2062"
Comparison complete. Found 2 matches out of 1491 pieces
Total Matchrate : 0.13%
```

## Future Improvements / Conclusion
While the current implementation may offer limited benefits, potential enhancements could involve exploring v2 torrents, which offer hashes per file. However, the nature of OS torrents, typically comprising a single large image .iso file, may not fully exploit this v2 feature. Further experimentation might explore structuring torrents to encourage greater sharing of piece data among newer versions, such as by appending new or modified data at the end of the torrent. Nevertheless, considering the ultimately low cost of bandwidth, especially when a deploying to many machines within a LAN environment, the added complexity of such optimizations may not yield substantial value.