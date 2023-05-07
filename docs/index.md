# EPDx

EPDx is a library for parsing EPD files into a common exchange format.

The library is written in Rust, to allow for cross-language package distribution.
Currently, we support Javascript/Typescript, Python and Rust.

EPDx is part of a larger project of making life cycle assessments more accessible and transparent to building
professionals.

Read more about our [overall goal](https://github.com/ocni-dtu/life-cycle-formats)

## ILCD+EPD

The ILCD+EPD format is a digital format for EPDs.
Provided in either XML or JSON.

Initially EPDs were provided as PDFs.
That is not very convenient for automation processes and requires large amounts of manual work to process.

The ILCD (International Life Cycle Data System) is a data format developed by the European Commission to give LCA
practitioners a common digital format for life cycle assessments.
In order to integrate EPD specific information (e.g. scenarios, modules, type of data), extensions were added to the
ILCD format. The resulting format got named ILCD+EPD format.

The ECO Platform and Ökobau uses the format to store and expose EPDs through
the [soda4LCA](https://bitbucket.org/okusche/soda4lca/src/7.x-branch/) API.
The API makes it possible for everyone to search and download tons of EPDs to be used free of charge.

The ILCD+EPD format is a node based format. It means that each EPD consists of several layers of nodes that contain
different information.
The first node contains a summary of the EPD, with references to the nodes.
By drilling down the tree of nodes, you have access to more and more specific information, such as EPD manufacturer,
functional unit(s), etc.

Read more about ILCD at the [European Platform on Life Cycle Assessment](https://eplca.jrc.ec.europa.eu/)

## Install Packages

=== "JavaScript"

    ```bash
    npm install epdx
    ```

=== "Python"

    ```bash
    pip install epdx
    ```

=== "Rust"

    ```bash
    cargo add epdx
    ```
