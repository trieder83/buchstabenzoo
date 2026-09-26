using UnityEngine;

/// <summary>
/// Attach to an empty GameObject in the scene. On Start, it builds
/// the entire zoo with placeholder voxel-style geometry so you can
/// hit Play immediately without importing any assets.
/// </summary>
public class ZooWorldBuilder : MonoBehaviour
{
    [Header("Materials (auto-created if null)")]
    public Material groundMat;
    public Material fenceMat;
    public Material waterMat;

    void Start()
    {
        CreateMaterials();
        CreateGround();
        CreateEnclosures();
        CreateAnimals();
        CreatePirateShip();
        SetupCamera();
        SetupLighting();
    }

    void CreateMaterials()
    {
        Shader lit = Shader.Find("Universal Render Pipeline/Lit")
                  ?? Shader.Find("Standard");

        if (groundMat == null)
        {
            groundMat = new Material(lit);
            groundMat.color = new Color(0.4f, 0.7f, 0.3f); // grass green
        }
        if (fenceMat == null)
        {
            fenceMat = new Material(lit);
            fenceMat.color = new Color(0.6f, 0.4f, 0.2f); // wood brown
        }
        if (waterMat == null)
        {
            waterMat = new Material(lit);
            waterMat.color = new Color(0.2f, 0.5f, 0.9f, 0.7f); // blue water
        }
    }

    // ── Ground ──────────────────────────────────────────────

    void CreateGround()
    {
        var ground = GameObject.CreatePrimitive(PrimitiveType.Cube);
        ground.name = "Ground";
        ground.transform.position = new Vector3(0, -0.5f, 0);
        ground.transform.localScale = new Vector3(60, 1, 60);
        ground.GetComponent<Renderer>().material = groundMat;

        // Path (darker strip)
        var path = GameObject.CreatePrimitive(PrimitiveType.Cube);
        path.name = "MainPath";
        path.transform.position = new Vector3(0, 0.01f, 0);
        path.transform.localScale = new Vector3(4, 0.05f, 50);
        var pathMat = new Material(groundMat);
        pathMat.color = new Color(0.8f, 0.7f, 0.5f);
        path.GetComponent<Renderer>().material = pathMat;
    }

    // ── Enclosures ──────────────────────────────────────────

    struct EnclosureData
    {
        public string name;
        public Animal.AnimalType type;
        public Vector3 position;
        public Color groundColor;
        public bool hasWater;
    }

    void CreateEnclosures()
    {
        EnclosureData[] enclosures = new[]
        {
            new EnclosureData { name = "Nilpferd",  type = Animal.AnimalType.Hippo,    position = new Vector3(-12, 0, 15), groundColor = new Color(0.5f, 0.65f, 0.4f), hasWater = true },
            new EnclosureData { name = "Panda",     type = Animal.AnimalType.Panda,    position = new Vector3(12, 0, 15),  groundColor = new Color(0.35f, 0.6f, 0.3f), hasWater = false },
            new EnclosureData { name = "Zebra",     type = Animal.AnimalType.Zebra,    position = new Vector3(-12, 0, 0),  groundColor = new Color(0.6f, 0.55f, 0.35f), hasWater = false },
            new EnclosureData { name = "Koala",     type = Animal.AnimalType.Koala,    position = new Vector3(12, 0, 0),   groundColor = new Color(0.35f, 0.55f, 0.3f), hasWater = false },
            new EnclosureData { name = "Elefant",   type = Animal.AnimalType.Elephant, position = new Vector3(-12, 0, -15), groundColor = new Color(0.55f, 0.5f, 0.35f), hasWater = true },
            new EnclosureData { name = "Goldfisch", type = Animal.AnimalType.Goldfish, position = new Vector3(12, 0, -15), groundColor = new Color(0.3f, 0.5f, 0.7f), hasWater = true },
        };

        foreach (var data in enclosures)
        {
            BuildEnclosure(data);
        }
    }

    void BuildEnclosure(EnclosureData data)
    {
        var parent = new GameObject($"Enclosure_{data.name}");
        parent.transform.position = data.position;

        // Floor
        var floor = GameObject.CreatePrimitive(PrimitiveType.Cube);
        floor.name = "Floor";
        floor.transform.SetParent(parent.transform);
        floor.transform.localPosition = new Vector3(0, 0.02f, 0);
        floor.transform.localScale = new Vector3(8, 0.04f, 8);
        var floorMat = new Material(groundMat);
        floorMat.color = data.groundColor;
        floor.GetComponent<Renderer>().material = floorMat;

        // Fence posts
        BuildFence(parent.transform, 8f, 8f);

        // Water pool
        if (data.hasWater)
        {
            var pool = GameObject.CreatePrimitive(PrimitiveType.Cylinder);
            pool.name = "WaterPool";
            pool.transform.SetParent(parent.transform);
            pool.transform.localPosition = new Vector3(1.5f, 0.1f, 1.5f);
            pool.transform.localScale = new Vector3(2.5f, 0.15f, 2.5f);
            pool.GetComponent<Renderer>().material = waterMat;
        }

        // Name sign
        var sign = CreateSign(data.name);
        sign.transform.SetParent(parent.transform);
        sign.transform.localPosition = new Vector3(0, 2.5f, -4.2f);

        // Enclosure component
        var enc = parent.AddComponent<Enclosure>();
        enc.enclosureName = data.name;
        enc.acceptedAnimalType = data.type;
        enc.nameSign = sign;

        // Spawn point
        var spawnPoint = new GameObject("SpawnPoint");
        spawnPoint.transform.SetParent(parent.transform);
        spawnPoint.transform.localPosition = Vector3.zero;
        enc.animalSpawnPoint = spawnPoint.transform;

        // Trigger collider for detecting animals
        var triggerObj = new GameObject("Trigger");
        triggerObj.transform.SetParent(parent.transform);
        triggerObj.transform.localPosition = Vector3.zero;
        var box = triggerObj.AddComponent<BoxCollider>();
        box.size = new Vector3(8, 3, 8);
        box.isTrigger = true;
        triggerObj.AddComponent<Enclosure>().acceptedAnimalType = data.type;
    }

    void BuildFence(Transform parent, float width, float depth)
    {
        float postSpacing = 2f;
        float postHeight = 1.2f;

        for (float x = -width / 2; x <= width / 2; x += postSpacing)
        {
            CreateFencePost(parent, new Vector3(x, postHeight / 2, -depth / 2));
            CreateFencePost(parent, new Vector3(x, postHeight / 2, depth / 2));
        }
        for (float z = -depth / 2; z <= depth / 2; z += postSpacing)
        {
            CreateFencePost(parent, new Vector3(-width / 2, postHeight / 2, z));
            CreateFencePost(parent, new Vector3(width / 2, postHeight / 2, z));
        }

        // Horizontal rails
        CreateRail(parent, new Vector3(0, 0.8f, -depth / 2), new Vector3(width, 0.15f, 0.15f));
        CreateRail(parent, new Vector3(0, 0.8f, depth / 2), new Vector3(width, 0.15f, 0.15f));
        CreateRail(parent, new Vector3(-width / 2, 0.8f, 0), new Vector3(0.15f, 0.15f, depth));
        CreateRail(parent, new Vector3(width / 2, 0.8f, 0), new Vector3(0.15f, 0.15f, depth));
    }

    void CreateFencePost(Transform parent, Vector3 pos)
    {
        var post = GameObject.CreatePrimitive(PrimitiveType.Cube);
        post.name = "FencePost";
        post.transform.SetParent(parent);
        post.transform.localPosition = pos;
        post.transform.localScale = new Vector3(0.2f, 1.2f, 0.2f);
        post.GetComponent<Renderer>().material = fenceMat;
    }

    void CreateRail(Transform parent, Vector3 pos, Vector3 scale)
    {
        var rail = GameObject.CreatePrimitive(PrimitiveType.Cube);
        rail.name = "FenceRail";
        rail.transform.SetParent(parent);
        rail.transform.localPosition = pos;
        rail.transform.localScale = scale;
        rail.GetComponent<Renderer>().material = fenceMat;
    }

    // ── Animals (voxel-style placeholder boxes) ─────────────

    struct AnimalData
    {
        public string name;
        public string displayName;
        public Animal.AnimalType type;
        public Vector3 position;
        public Vector3 bodyScale;
        public Color bodyColor;
        public string[] food;
    }

    void CreateAnimals()
    {
        AnimalData[] animals = new[]
        {
            new AnimalData {
                name = "Hippo1", displayName = "Nilpferd", type = Animal.AnimalType.Hippo,
                position = new Vector3(-5, 0, 20), bodyScale = new Vector3(1.8f, 1.2f, 2.5f),
                bodyColor = new Color(0.5f, 0.45f, 0.5f),
                food = new[] { "Gras", "Heu" }
            },
            new AnimalData {
                name = "Hippo2", displayName = "Nilpferd", type = Animal.AnimalType.Hippo,
                position = new Vector3(-3, 0, 22), bodyScale = new Vector3(1.6f, 1.0f, 2.2f),
                bodyColor = new Color(0.55f, 0.48f, 0.52f),
                food = new[] { "Gras", "Heu" }
            },
            new AnimalData {
                name = "Hippo3", displayName = "Nilpferd", type = Animal.AnimalType.Hippo,
                position = new Vector3(-7, 0, 21), bodyScale = new Vector3(1.5f, 0.9f, 2.0f),
                bodyColor = new Color(0.48f, 0.43f, 0.48f),
                food = new[] { "Gras", "Heu" }
            },
            new AnimalData {
                name = "Panda", displayName = "Panda", type = Animal.AnimalType.Panda,
                position = new Vector3(5, 0, 20), bodyScale = new Vector3(1.2f, 1.4f, 1.2f),
                bodyColor = Color.white,
                food = new[] { "Bambus" }
            },
            new AnimalData {
                name = "Zebra", displayName = "Zebra", type = Animal.AnimalType.Zebra,
                position = new Vector3(0, 0, -22), bodyScale = new Vector3(1.0f, 1.5f, 2.0f),
                bodyColor = Color.white,
                food = new[] { "Blätter", "Gebüsch", "Gras" }
            },
            new AnimalData {
                name = "Koala", displayName = "Koala", type = Animal.AnimalType.Koala,
                position = new Vector3(18, 0, 5), bodyScale = new Vector3(0.8f, 0.9f, 0.8f),
                bodyColor = new Color(0.6f, 0.6f, 0.55f),
                food = new[] { "Eukalyptus" }
            },
            new AnimalData {
                name = "Elephant", displayName = "Elefant", type = Animal.AnimalType.Elephant,
                position = new Vector3(-18, 0, -5), bodyScale = new Vector3(2.0f, 2.2f, 3.0f),
                bodyColor = new Color(0.55f, 0.55f, 0.55f),
                food = new[] { "Bäume", "Wasser", "Heu", "Baumstämme" }
            },
            new AnimalData {
                name = "Monkey", displayName = "Affe", type = Animal.AnimalType.Monkey,
                position = new Vector3(20, 0, -20), bodyScale = new Vector3(0.7f, 1.0f, 0.6f),
                bodyColor = new Color(0.55f, 0.35f, 0.2f),
                food = new[] { "Banane", "Früchte" }
            },
        };

        var animalsParent = new GameObject("Animals");

        foreach (var data in animals)
        {
            BuildAnimal(data, animalsParent.transform);
        }
    }

    void BuildAnimal(AnimalData data, Transform parent)
    {
        Shader lit = Shader.Find("Universal Render Pipeline/Lit")
                  ?? Shader.Find("Standard");

        // Parent object
        var animalObj = new GameObject(data.name);
        animalObj.transform.SetParent(parent);
        animalObj.transform.position = data.position;
        animalObj.layer = LayerMask.NameToLayer("Default");

        // Body (main box — voxel style)
        var body = GameObject.CreatePrimitive(PrimitiveType.Cube);
        body.name = "Body";
        body.transform.SetParent(animalObj.transform);
        body.transform.localPosition = new Vector3(0, data.bodyScale.y / 2, 0);
        body.transform.localScale = data.bodyScale;
        var bodyMat = new Material(lit);
        bodyMat.color = data.bodyColor;
        body.GetComponent<Renderer>().material = bodyMat;

        // Head (smaller cube)
        var head = GameObject.CreatePrimitive(PrimitiveType.Cube);
        head.name = "Head";
        head.transform.SetParent(animalObj.transform);
        float headSize = data.bodyScale.x * 0.6f;
        head.transform.localPosition = new Vector3(0, data.bodyScale.y * 0.8f, data.bodyScale.z / 2 + headSize * 0.3f);
        head.transform.localScale = new Vector3(headSize, headSize, headSize);
        head.GetComponent<Renderer>().material = bodyMat;

        // Legs (4 small cubes)
        float legWidth = data.bodyScale.x * 0.2f;
        float legHeight = data.bodyScale.y * 0.4f;
        Vector3[] legOffsets = new[]
        {
            new Vector3(-data.bodyScale.x * 0.3f, legHeight / 2, -data.bodyScale.z * 0.3f),
            new Vector3(data.bodyScale.x * 0.3f, legHeight / 2, -data.bodyScale.z * 0.3f),
            new Vector3(-data.bodyScale.x * 0.3f, legHeight / 2, data.bodyScale.z * 0.3f),
            new Vector3(data.bodyScale.x * 0.3f, legHeight / 2, data.bodyScale.z * 0.3f),
        };

        var legMat = new Material(lit);
        legMat.color = data.bodyColor * 0.8f;

        foreach (var offset in legOffsets)
        {
            var leg = GameObject.CreatePrimitive(PrimitiveType.Cube);
            leg.name = "Leg";
            leg.transform.SetParent(animalObj.transform);
            leg.transform.localPosition = offset;
            leg.transform.localScale = new Vector3(legWidth, legHeight, legWidth);
            leg.GetComponent<Renderer>().material = legMat;
        }

        // Collider on parent for picking
        var col = animalObj.AddComponent<BoxCollider>();
        col.center = new Vector3(0, data.bodyScale.y / 2, 0);
        col.size = data.bodyScale * 1.2f;

        // Rigidbody for trigger detection
        var rb = animalObj.AddComponent<Rigidbody>();
        rb.isKinematic = true;

        // Animal script
        var animal = animalObj.AddComponent<Animal>();
        animal.animalName = data.name;
        animal.displayName = data.displayName;
        animal.type = data.type;
        animal.acceptedFood = data.food;
        animal.wanderRadius = 4f;
    }

    // ── Pirate Ship (food key location) ─────────────────────

    void CreatePirateShip()
    {
        Shader lit = Shader.Find("Universal Render Pipeline/Lit")
                  ?? Shader.Find("Standard");

        var ship = new GameObject("PirateShip");
        ship.transform.position = new Vector3(25, 0, 0);

        // Hull
        var hull = GameObject.CreatePrimitive(PrimitiveType.Cube);
        hull.name = "Hull";
        hull.transform.SetParent(ship.transform);
        hull.transform.localPosition = new Vector3(0, 1, 0);
        hull.transform.localScale = new Vector3(4, 2, 8);
        var hullMat = new Material(lit);
        hullMat.color = new Color(0.45f, 0.25f, 0.1f);
        hull.GetComponent<Renderer>().material = hullMat;

        // Mast
        var mast = GameObject.CreatePrimitive(PrimitiveType.Cube);
        mast.name = "Mast";
        mast.transform.SetParent(ship.transform);
        mast.transform.localPosition = new Vector3(0, 5, 0);
        mast.transform.localScale = new Vector3(0.3f, 8, 0.3f);
        mast.GetComponent<Renderer>().material = hullMat;

        // Sail
        var sail = GameObject.CreatePrimitive(PrimitiveType.Cube);
        sail.name = "Sail";
        sail.transform.SetParent(ship.transform);
        sail.transform.localPosition = new Vector3(0, 6, 0.5f);
        sail.transform.localScale = new Vector3(0.1f, 4, 3);
        var sailMat = new Material(lit);
        sailMat.color = Color.white;
        sail.GetComponent<Renderer>().material = sailMat;

        // Skull flag (small dark cube)
        var flag = GameObject.CreatePrimitive(PrimitiveType.Cube);
        flag.name = "PirateFlag";
        flag.transform.SetParent(ship.transform);
        flag.transform.localPosition = new Vector3(0, 9.2f, 0);
        flag.transform.localScale = new Vector3(0.05f, 1, 1.2f);
        var flagMat = new Material(lit);
        flagMat.color = Color.black;
        flag.GetComponent<Renderer>().material = flagMat;
    }

    // ── Camera & Lighting ───────────────────────────────────

    void SetupCamera()
    {
        var cam = Camera.main;
        if (cam == null) return;

        // Isometric-like angle
        cam.transform.position = new Vector3(0, 25, -20);
        cam.transform.rotation = Quaternion.Euler(50, 0, 0);

        // Add controllers
        if (cam.GetComponent<CameraController>() == null)
            cam.gameObject.AddComponent<CameraController>();

        var input = cam.GetComponent<TouchInputController>();
        if (input == null)
            input = cam.gameObject.AddComponent<TouchInputController>();
        input.mainCamera = cam;
        input.interactableLayer = ~0; // all layers for now
    }

    void SetupLighting()
    {
        var sun = GameObject.Find("Directional Light");
        if (sun != null)
        {
            sun.transform.rotation = Quaternion.Euler(45, 30, 0);
            var light = sun.GetComponent<Light>();
            if (light != null)
            {
                light.intensity = 1.2f;
                light.color = new Color(1f, 0.95f, 0.85f);
            }
        }
    }

    // ── Helper ──────────────────────────────────────────────

    GameObject CreateSign(string text)
    {
        Shader lit = Shader.Find("Universal Render Pipeline/Lit")
                  ?? Shader.Find("Standard");

        var sign = new GameObject($"Sign_{text}");

        // Post
        var post = GameObject.CreatePrimitive(PrimitiveType.Cube);
        post.transform.SetParent(sign.transform);
        post.transform.localPosition = new Vector3(0, 1, 0);
        post.transform.localScale = new Vector3(0.15f, 2f, 0.15f);
        post.GetComponent<Renderer>().material = fenceMat;

        // Board
        var board = GameObject.CreatePrimitive(PrimitiveType.Cube);
        board.transform.SetParent(sign.transform);
        board.transform.localPosition = new Vector3(0, 2.2f, 0);
        board.transform.localScale = new Vector3(2.5f, 0.8f, 0.1f);
        var boardMat = new Material(lit);
        boardMat.color = new Color(0.85f, 0.75f, 0.55f);
        board.GetComponent<Renderer>().material = boardMat;

        // TextMesh for the name
        var textObj = new GameObject("Text");
        textObj.transform.SetParent(sign.transform);
        textObj.transform.localPosition = new Vector3(0, 2.2f, -0.06f);
        var tm = textObj.AddComponent<TextMesh>();
        tm.text = text;
        tm.fontSize = 40;
        tm.characterSize = 0.1f;
        tm.anchor = TextAnchor.MiddleCenter;
        tm.alignment = TextAlignment.Center;
        tm.color = new Color(0.2f, 0.1f, 0.05f);

        return sign;
    }
}
